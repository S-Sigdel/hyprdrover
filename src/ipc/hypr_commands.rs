use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::Command;
use std::fs;

// --- Data Models (matching hyprctl -j output) ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HyprWorkspaceRef {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HyprActiveWorkspace {
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

    /// Full argv-style command used to launch the application.
    /// This is required to correctly restore PWAs, Electron apps,
    /// and browser-based app runtimes.
    #[serde(default)]
    pub command: Option<Vec<String>>,

    /// Fallback executable path from /proc/<pid>/exe.
    /// Used only if command is unavailable.
    #[serde(default)]
    pub exe_path: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionSnapshot {
    pub clients: Vec<HyprClient>,
    pub workspaces: Vec<HyprWorkspace>,
    pub monitors: Vec<HyprMonitor>,
}

// --- Implementation ---

/// Execute a hyprctl command and return the output as a string
fn run_hyprctl(args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("hyprctl")
        .arg("-j")
        .args(args)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "hyprctl failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

/// Get all open windows (clients)
fn get_clients() -> Result<Vec<HyprClient>, Box<dyn Error>> {
    let json = run_hyprctl(&["clients"])?;
    Ok(serde_json::from_str(&json)?)
}

/// Get all active workspaces
fn get_workspaces() -> Result<Vec<HyprWorkspace>, Box<dyn Error>> {
    let json = run_hyprctl(&["workspaces"])?;
    Ok(serde_json::from_str(&json)?)
}

/// Get all connected monitors
fn get_monitors() -> Result<Vec<HyprMonitor>, Box<dyn Error>> {
    let json = run_hyprctl(&["monitors"])?;
    Ok(serde_json::from_str(&json)?)
}

/// Get the active workspace for the currently focused monitor
pub fn get_active_workspace() -> Result<HyprActiveWorkspace, Box<dyn Error>> {
    let json = run_hyprctl(&["activeworkspace"])?;
    Ok(serde_json::from_str(&json)?)
}

/// Capture the entire current state of Hyprland
pub fn capture_state() -> Result<SessionSnapshot, Box<dyn Error>> {
    let mut clients = get_clients()?;

    for client in &mut clients {
        let cmdline_path = format!("/proc/{}/cmdline", client.pid);
        let exe_path = format!("/proc/{}/exe", client.pid);

        // Prefer full argv from /proc/<pid>/cmdline
        if let Ok(bytes) = fs::read(&cmdline_path) {
            let args: Vec<String> = bytes
                .split(|b| *b == 0)
                .filter(|s| !s.is_empty())
                .map(|s| String::from_utf8_lossy(s).into_owned())
                .collect();

            if !args.is_empty() {
                client.command = Some(args);
                continue;
            }
        }

        // Fallback: kernel-reported executable path
        if let Ok(path) = fs::read_link(&exe_path) {
            client.exe_path = Some(path.to_string_lossy().into_owned());
        }
    }

    Ok(SessionSnapshot {
        clients,
        workspaces: get_workspaces()?,
        monitors: get_monitors()?,
    })
}

// --- Dispatch Commands (Actions) ---

/// Execute a raw hyprctl dispatch command
pub fn dispatch(command: &str) -> Result<(), Box<dyn Error>> {
    let args: Vec<&str> = command.split_whitespace().collect();
    let output = Command::new("hyprctl")
        .arg("dispatch")
        .args(&args)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Dispatch failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}

/// Move a specific window to a workspace (silently)
pub fn move_window_to_workspace(address: &str, workspace_id: i32) -> Result<(), Box<dyn Error>> {
    let cmd = format!("movetoworkspacesilent {},address:{}", workspace_id, address);
    dispatch(&cmd)
}

/// Focus a specific window
#[allow(dead_code)]
pub fn focus_window(address: &str) -> Result<(), Box<dyn Error>> {
    let cmd = format!("focuswindow address:{}", address);
    dispatch(&cmd)
}

/// Move a window to a specific pixel coordinate
pub fn move_window_pixel(address: &str, x: i32, y: i32) -> Result<(), Box<dyn Error>> {
    let cmd = format!("movewindowpixel exact {} {},address:{}", x, y, address);
    dispatch(&cmd)
}

/// Resize a window to specific dimensions
pub fn resize_window_pixel(address: &str, width: i32, height: i32) -> Result<(), Box<dyn Error>> {
    let cmd = format!(
        "resizewindowpixel exact {} {},address:{}",
        width, height, address
    );
    dispatch(&cmd)
}
