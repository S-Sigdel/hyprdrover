use std::process::{Command, Stdio};
use std::error::Error;

/// Spawns a process in the background (detached)
#[allow(dead_code)]
pub fn spawn_process(command: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    Command::new(command)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_process_echo() {
        let result = spawn_process("echo", &["hello"]);
        assert!(result.is_ok());
    }
}
