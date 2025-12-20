mod config;
mod ipc;
mod restore;
mod state;

use config::Config;
use state::SessionManager;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::default();
    let manager = SessionManager::new(config.clone());

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "--save" => {
            let name = args.get(2).map(|s| s.as_str());
            match manager.snapshot(name) {
                Ok(path) => println!("Session saved to: {}", path.display()),
                Err(e) => eprintln!("Error saving session: {}", e),
            }
        }
        "--load" => {
            let path = if let Some(arg) = args.get(2) {
                let path = PathBuf::from(arg);
                if path.exists() {
                    path
                } else {
                    // Try looking in session directory
                    let session_dir = PathBuf::from(&config.session_dir);
                    let named_path = session_dir.join(arg).with_extension("json");
                    if named_path.exists() {
                        named_path
                    } else {
                        // Try without adding extension if user provided it
                        let named_path_exact = session_dir.join(arg);
                        if named_path_exact.exists() {
                            named_path_exact
                        } else {
                            eprintln!("Session file not found: {}", arg);
                            return;
                        }
                    }
                }
            } else {
                // Load latest
                match manager.list_sessions() {
                    Ok(sessions) => {
                        if let Some(latest) = sessions.first() {
                            println!(
                                "No file specified, loading latest session: {}",
                                latest.display()
                            );
                            latest.clone()
                        } else {
                            eprintln!("No saved sessions found.");
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing sessions: {}", e);
                        return;
                    }
                }
            };

            if let Err(e) = manager.restore(&path) {
                eprintln!("Error restoring session: {}", e);
            } else {
                println!("Session restored successfully.");
            }
        }
        "--list" => match manager.list_sessions() {
            Ok(sessions) => {
                if sessions.is_empty() {
                    println!("No saved sessions found.");
                } else {
                    println!("Saved sessions:");
                    for session in sessions {
                        println!("  {}", session.display());
                    }
                }
            }
            Err(e) => eprintln!("Error listing sessions: {}", e),
        },
        "--install" => {
            if let Err(e) = install_binary() {
                eprintln!("Error installing binary: {}", e);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage: hyprdrover [COMMAND]");
    println!("Commands:");
    println!("  --save [NAME]       Snapshot the current session (optional name)");
    println!("  --load [NAME|FILE]  Restore a session (by name or path, defaults to latest)");
    println!("  --list              List all saved sessions");
    println!("  --install           Install the binary to ~/.local/bin/");
}

fn install_binary() -> Result<(), Box<dyn std::error::Error>> {
    let current_exe = env::current_exe()?;
    let home_dir = env::var("HOME")?;
    let target_dir = PathBuf::from(home_dir).join(".local/bin");

    if !target_dir.exists() {
        std::fs::create_dir_all(&target_dir)?;
    }

    let target_path = target_dir.join("hyprdrover");

    std::fs::copy(&current_exe, &target_path)?;

    println!("Successfully installed to {}", target_path.display());
    println!("Ensure {} is in your PATH.", target_dir.display());

    Ok(())
}
