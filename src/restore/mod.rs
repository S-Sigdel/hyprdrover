pub mod position;

use crate::ipc::{self, SessionSnapshot};
use std::error::Error;

/// Orchestrates the restoration of a session
pub fn restore_session(snapshot: &SessionSnapshot) -> Result<(), Box<dyn Error>> {
    // 1. Get current state
    let current_state = ipc::capture_state()?;
    let mut available_clients = current_state.clients;

    // Track restored windows to avoid double-matching later
    let mut restored_addresses = std::collections::HashSet::new();
    // Track windows that need to be positioned after launch
    let mut pending_launches = Vec::new();

    // 2. Match and restore existing windows
    for saved_client in &snapshot.clients {
        // Try to find a matching client in the current session
        if let Some(index) = available_clients
            .iter()
            .position(|c| c.class == saved_client.class)
        {
            let current_client = available_clients.remove(index);
            println!(
                "   Restoring window: {} ({})",
                current_client.class, current_client.title
            );

            position::restore_window_position(&current_client, saved_client)?;
            restored_addresses.insert(current_client.address.clone());
        } else {
            println!("   ⚠️ Window missing: {}", saved_client.class);

            // Notify user
            let _ = std::process::Command::new("notify-send")
                .arg("Restoring Session")
                .arg(format!("Launching {}...", saved_client.class))
                .spawn();

            // Launch app on workspace
            let command = if let Some(path) = &saved_client.exec_path {
                path.clone()
            } else {
                // Fallback Heuristic: Use initial_class or class, converted to lowercase
                let raw_name = if !saved_client.initial_class.is_empty() {
                    &saved_client.initial_class
                } else {
                    &saved_client.class
                };
                resolve_command(raw_name)
            };

            println!("      -> Launching: {}", command);

            // We use std::process::Command directly here instead of ipc::dispatch
            // to ensure the "[workspace X silent] cmd" string is passed as a single argument
            // to the 'exec' dispatcher. ipc::dispatch splits by whitespace which breaks the rule syntax.
            let exec_arg = format!(
                "[workspace {} silent] {}",
                saved_client.workspace.id, command
            );

            let output = std::process::Command::new("hyprctl")
                .arg("dispatch")
                .arg("exec")
                .arg(&exec_arg)
                .output();

            match output {
                Ok(out) => {
                    if !out.status.success() {
                        eprintln!(
                            "Failed to launch {}: {}",
                            command,
                            String::from_utf8_lossy(&out.stderr)
                        );
                    } else {
                        // Add to pending list for post-launch positioning
                        pending_launches.push(saved_client);
                    }
                }
                Err(e) => eprintln!("Failed to execute hyprctl: {}", e),
            }
        }
    }

    // 3. Post-launch positioning
    if !pending_launches.is_empty() {
        println!("   Waiting for applications to start (2s)...");
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Refresh state to find newly launched windows
        let new_state = ipc::capture_state()?;
        let new_clients = new_state.clients;

        for saved_client in pending_launches {
            // Find a matching client that hasn't been restored yet
            // We match by class and workspace ID since we launched it there
            if let Some(current_client) = new_clients.iter().find(|c| {
                c.class == saved_client.class
                    && c.workspace.id == saved_client.workspace.id
                    && !restored_addresses.contains(&c.address)
            }) {
                println!("   Positioning launched window: {}", saved_client.class);
                if let Err(e) = position::restore_window_position(current_client, saved_client) {
                    eprintln!("Failed to position {}: {}", saved_client.class, e);
                } else {
                    restored_addresses.insert(current_client.address.clone());
                }
            } else {
                eprintln!(
                    "   Could not find launched window for positioning: {}",
                    saved_client.class
                );
            }
        }
    }

    Ok(())
}

fn resolve_command(class: &str) -> String {
    let lower = class.to_lowercase();
    match lower.as_str() {
        "brave-browser" => "brave".to_string(),
        "code" => "code".to_string(), // VS Code often has class "Code"
        "google-chrome" => "google-chrome-stable".to_string(),
        "com.mitchellh.ghostty" => "ghostty".to_string(),
        _ => lower,
    }
}
