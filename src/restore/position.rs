use crate::ipc::{self, HyprClient};
use std::error::Error;

/// Restores the position and workspace of a single window
pub fn restore_window_position(
    current_client: &HyprClient,
    saved_client: &HyprClient,
) -> Result<(), Box<dyn Error>> {
    // Move to workspace
    if current_client.workspace.id != saved_client.workspace.id {
        ipc::move_window_to_workspace(&current_client.address, saved_client.workspace.id)?;
    }

    // Move to position & Resize
    if saved_client.floating {
        if !current_client.floating {
            ipc::dispatch(&format!(
                "togglefloating address:{}",
                current_client.address
            ))?;
        }
        ipc::move_window_pixel(
            &current_client.address,
            saved_client.at[0],
            saved_client.at[1],
        )?;
        ipc::resize_window_pixel(
            &current_client.address,
            saved_client.size[0],
            saved_client.size[1],
        )?;
    } else {
        // Saved as tiled
        if current_client.floating {
            ipc::dispatch(&format!(
                "togglefloating address:{}",
                current_client.address
            ))?;
        }
        // For tiled windows, we can't easily force pixel positions without floating them.
        // We just move them to the workspace for now.
    }

    Ok(())
}
