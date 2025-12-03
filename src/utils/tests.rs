#[cfg(test)]
mod tests {
    use crate::utils::exec::spawn_process;

    #[test]
    fn test_spawn_process_echo() {
        // This is a bit tricky to test since it's detached, but we can try running a simple command
        // that has a side effect, or just ensure it doesn't panic.
        let result = spawn_process("echo", &["hello"]);
        assert!(result.is_ok());
    }
}
