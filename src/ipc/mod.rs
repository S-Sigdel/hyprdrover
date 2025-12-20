pub mod hypr_commands;

// Re-export the actual functions and structs we created
pub use hypr_commands::{
    capture_state, dispatch, get_active_workspace, move_window_pixel, move_window_to_workspace,
    resize_window_pixel, HyprClient, SessionSnapshot,
};
