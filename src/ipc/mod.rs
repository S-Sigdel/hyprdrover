pub mod hypr_commands;

// Re-export the actual functions and structs we created
pub use hypr_commands::{
    capture_state,
    dispatch,
    move_window_to_workspace,
    move_window_pixel,
    resize_window_pixel,
    SessionSnapshot,
    HyprClient,
};
