use serde::{Deserialize, Serialize};
use std::process::Command;
use std::error::Error;

// --- Data Models (matching hyprctl -j output) ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HyprWorkspaceRef {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HyprClient {
    pub address: String,
    pub at: [i32; 2],
    pub size: [i32; 2],
    pub workspace: HyprWorkspaceRef,
    pub class: String,
    pub title: String,
    pub initial_class: String,
    pub initial_title: String,
    pub floating: bool,
    pub pinned: bool,
    pub monitor: i64,
    pub fullscreen: i32, // 0: none, 1: maximized, 2: fullscreen
    pub xwayland: bool,
    pub pid: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HyprWorkspace {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    pub windows: i32,
    pub hasfullscreen: bool,
    pub lastwindow: String,
    pub lastwindowtitle: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HyprMonitor {
    pub id: i64,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub refresh_rate: f32,
    pub x: i32,
    pub y: i32,
    pub active_workspace: HyprWorkspaceRef,
}

// --- Helper Struct for the full snapshot ---

#[derive(Debug, Serialize)]
pub struct SessionSnapshot {
    pub clients: Vec<HyprClient>,
    pub workspaces: Vec<HyprWorkspace>,
    pub monitors: Vec<HyprMonitor>,
}

// --- Implementation ---

/// Execute a hyprctl command and return the output as a string
fn run_hyprctl(args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("hyprctl")
        .arg("-j") // Request JSON output
        .args(args)
        .output()?;

    if !output.status.success() {
        return Err(format!("hyprctl failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

/// Get all open windows (clients)
fn get_clients() -> Result<Vec<HyprClient>, Box<dyn Error>> {
    let json = run_hyprctl(&["clients"])?;
    let clients: Vec<HyprClient> = serde_json::from_str(&json)?;
    Ok(clients)
}

/// Get all active workspaces
fn get_workspaces() -> Result<Vec<HyprWorkspace>, Box<dyn Error>> {
    let json = run_hyprctl(&["workspaces"])?;
    let workspaces: Vec<HyprWorkspace> = serde_json::from_str(&json)?;
    Ok(workspaces)
}

/// Get all connected monitors
fn get_monitors() -> Result<Vec<HyprMonitor>, Box<dyn Error>> {
    let json = run_hyprctl(&["monitors"])?;
    let monitors: Vec<HyprMonitor> = serde_json::from_str(&json)?;
    Ok(monitors)
}

/// Capture the entire current state of Hyprland
pub fn capture_state() -> Result<SessionSnapshot, Box<dyn Error>> {
    Ok(SessionSnapshot {
        clients: get_clients()?,
        workspaces: get_workspaces()?,
        monitors: get_monitors()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_client() {
        let json = r#"{
            "address": "0x1234",
            "at": [10, 10],
            "size": [800, 600],
            "workspace": { "id": 1, "name": "1" },
            "class": "kitty",
            "title": "terminal",
            "initialClass": "kitty",
            "initialTitle": "terminal",
            "floating": false,
            "pinned": false,
            "monitor": 0,
            "fullscreen": 0,
            "xwayland": false,
            "pid": 1234
        }"#;

        let client: HyprClient = serde_json::from_str(json).expect("Failed to deserialize client");
        
        assert_eq!(client.address, "0x1234");
        assert_eq!(client.class, "kitty");
        assert_eq!(client.workspace.id, 1);
        assert_eq!(client.size, [800, 600]);
    }

    #[test]
    fn test_deserialize_workspace() {
        let json = r#"{
            "id": 1,
            "name": "1",
            "monitor": "eDP-1",
            "windows": 5,
            "hasfullscreen": false,
            "lastwindow": "0x1234",
            "lastwindowtitle": "terminal"
        }"#;

        let ws: HyprWorkspace = serde_json::from_str(json).expect("Failed to deserialize workspace");
        
        assert_eq!(ws.id, 1);
        assert_eq!(ws.monitor, "eDP-1");
        assert_eq!(ws.windows, 5);
    }

    #[test]
    fn test_deserialize_monitor() {
        let json = r#"{
            "id": 0,
            "name": "eDP-1",
            "width": 1920,
            "height": 1080,
            "refreshRate": 60.0,
            "x": 0,
            "y": 0,
            "activeWorkspace": { "id": 1, "name": "1" }
        }"#;

        let mon: HyprMonitor = serde_json::from_str(json).expect("Failed to deserialize monitor");
        
        assert_eq!(mon.id, 0);
        assert_eq!(mon.width, 1920);
        assert_eq!(mon.active_workspace.id, 1);
    }
}
