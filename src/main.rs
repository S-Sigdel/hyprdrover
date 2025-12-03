mod ipc;
mod snapshot;
mod config;
mod state;
mod utils;

use config::Config;
use state::SessionManager;

fn main() {
    // 1. Initialize Config (Defaults for now)
    let config = Config::default();
    let manager = SessionManager::new(config);

    println!("Scanning Hyprland Session...");

    // 2. Use the SessionManager to handle the logic
    match manager.snapshot() {
        Ok(path) => {
            println!("Snapshot captured successfully.");
            println!("Session saved to: {}", path.display());
            
            // List recent sessions just to show it works
            if let Ok(sessions) = manager.list_sessions() {
                println!("\nTotal saved sessions: {}", sessions.len());
            }

            // 3. Run the Movement Test
            println!("\nRunning Movement Test...");
            if let Err(e) = run_movement_test() {
                eprintln!("Test failed: {}", e);
            }
        }
        Err(e) => eprintln!("Error capturing state: {}", e),
    }
}

/// A small script to test moving windows around
fn run_movement_test() -> Result<(), Box<dyn std::error::Error>> {
    use std::{thread, time};

    // Capture state to find a victim window
    let state = ipc::capture_state()?;
    
    // Find a window that isn't our own terminal (optional heuristic, but good practice)
    // For now, just pick the first one that looks like a normal window
    if let Some(client) = state.clients.first() {
        println!("Target found: [{}] {} ({})", client.class, client.title, client.address);
        println!("   Current Workspace: {}", client.workspace.id);

        let original_workspace = client.workspace.id;
        // Move to workspace 9 (usually empty) or just +1
        let target_workspace = if original_workspace == 9 { 1 } else { 9 };

        println!("   -> Moving to Workspace {} (Silent)...", target_workspace);
        ipc::move_window_to_workspace(&client.address, target_workspace)?;
        
        println!("   -> Waiting 2 seconds...");
        thread::sleep(time::Duration::from_secs(2));

        println!("   -> Moving back to Workspace {}...", original_workspace);
        ipc::move_window_to_workspace(&client.address, original_workspace)?;
        
        println!("   -> Focusing window...");
        ipc::focus_window(&client.address)?;

        println!("Test Complete: Window moved there and back again.");
    } else {
        println!("No windows found to test with.");
    }

    Ok(())
}
