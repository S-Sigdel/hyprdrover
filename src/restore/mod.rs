pub mod position;
pub mod spawn;

use std::error::Error;
use crate::ipc::{self, SessionSnapshot};

/// Orchestrates the restoration of a session
pub fn restore_session(snapshot: &SessionSnapshot) -> Result<(), Box<dyn Error>> {
    // 1. Get current state
    let current_state = ipc::capture_state()?;
    let mut available_clients = current_state.clients;

    // 2. Match and restore
    for saved_client in &snapshot.clients {
        // Try to find a matching client in the current session
        if let Some(index) = available_clients.iter().position(|c| {
            c.class == saved_client.class 
        }) {
            let current_client = available_clients.remove(index);
            println!("   Restoring window: {} ({})", current_client.class, current_client.title);

            position::restore_window_position(&current_client, saved_client)?;

        } else {
            println!("   ⚠️ Window missing: {}", saved_client.class);
            // Future: spawn::spawn_process(...)
        }
    }

    Ok(())
}
