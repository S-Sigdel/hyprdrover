pub mod hypr_commands;

// Re-export the actual functions and structs we created
pub use hypr_commands::{
    capture_state,
    dispatch,
    move_window_to_workspace,
    focus_window,
    SessionSnapshot,
    HyprClient,
    HyprWorkspace,
    HyprMonitor,
    HyprWorkspaceRef,
};
