use rgit::{Command, execute_command, parse_args};

#[test]
fn test_init_command_parsing() {
    let args = vec!["rgit".to_string(), "init".to_string()];

    let config = parse_args(args);

    assert_eq!(config.command, Command::Init);
    assert!(config.positional_args.is_empty());
}

#[test]
fn test_help_command_parsing() {
    let args = vec!["rgit".to_string(), "--help".to_string()];

    let config = parse_args(args);

    assert_eq!(config.command, Command::Help);
}
#[test]
fn test_init_creates_repository_structure() {
    // 1. Create a securely isolated temporary directory.
    //    This automatically deletes itself from your OS when `temp_dir` goes out of scope!
    let temp_dir = tempfile::tempdir().unwrap();
    let base_path = temp_dir.path();

    let mut output = Vec::new();

    // 2. Execute the command against our safe RAII-managed path
    let result = execute_command(&Command::Init, base_path, &mut output);

    // 3. Assertions
    assert!(result.is_ok());
    assert!(base_path.join(".rgit").exists());
    assert!(base_path.join(".rgit/objects").exists());
    assert!(base_path.join(".rgit/refs").exists());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Initializing repository"));
}
