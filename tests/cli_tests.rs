use rgit::{Command, execute_command, parse_args};
use std::{env, fs};

#[test]
fn test_init_command_parsing() {
    let args = vec!["rgit".to_string(), "init".to_string()];
    let mut output_buffer = Vec::new();

    let config = parse_args(args, &mut output_buffer);

    // Checks that the intent is properly registered
    assert_eq!(config.command, Command::Init);
    assert!(config.positional_args.is_empty());
}

#[test]
fn test_init_creates_directory_safely() {
    // Create a temporary sandbox directory for this test using standard library env
    let temp_dir = env::temp_dir().join(format!("rgit_test_{}", line!()));
    fs::create_dir_all(&temp_dir).unwrap();

    // Execute the command inside our sandboxed directory
    let result = execute_command(&Command::Init, temp_dir.clone());

    assert!(result.is_ok());
    assert!(
        temp_dir.join(".rgit").exists(),
        "The .rgit directory should have been created!"
    );

    // Cleanup after ourselves
    fs::remove_dir_all(temp_dir).unwrap();
}
