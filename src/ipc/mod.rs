pub mod hypr_commands;

// Re-export the actual functions and structs we created
pub use hypr_commands::{
    capture_state, 
    SessionSnapshot,
    HyprClient,
    HyprWorkspace,
    HyprMonitor,
    HyprWorkspaceRef,
};
