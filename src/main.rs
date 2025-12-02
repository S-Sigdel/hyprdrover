mod ipc;
mod snapshot;

fn main() {
    println!("Scanning Hyprland Session...");

    match ipc::capture_state() {
        Ok(snapshot) => {
            println!("Snapshot captured successfully.");
            println!("--------------------------------");
            
            println!("Monitors: {}", snapshot.monitors.len());
            for mon in &snapshot.monitors {
                println!("   - {} ({}x{} @ {:.2}Hz)", mon.name, mon.width, mon.height, mon.refresh_rate);
            }

            println!("\nWorkspaces: {}", snapshot.workspaces.len());
            for ws in &snapshot.workspaces {
                println!("   - ID {}: {} (on {})", ws.id, ws.name, ws.monitor);
            }

            println!("\nWindows: {}", snapshot.clients.len());
            for client in &snapshot.clients {
                // Truncate title if too long for display
                let display_title = if client.title.len() > 50 {
                    format!("{}...", &client.title[..50])
                } else {
                    client.title.clone()
                };
                
                println!(
                    "   - [{}] {}\n     -> Workspace: {} | Pos: {:?} | Size: {:?}", 
                    client.class, 
                    display_title,
                    client.workspace.id,
                    client.at,
                    client.size
                );
            }

            // Save to JSON
            println!("\nSaving session...");
            match snapshot::save_session_to_file(&snapshot) {
                Ok(path) => println!("Session saved to: {}", path),
                Err(e) => eprintln!("Failed to save session: {}", e),
            }
        }
        Err(e) => eprintln!("Error capturing state: {}", e),
    }
}
