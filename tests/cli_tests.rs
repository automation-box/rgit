use rgit::{Command, execute_command, parse_args};
use std::{env, fs};

#[test]
fn test_init_command_parsing() {
    let args = vec!["rgit".to_string(), "init".to_string()];

    // Look mom, no writer required!
    let config = parse_args(args);

    assert_eq!(config.command, Command::Init);
    assert!(config.positional_args.is_empty());
}

#[test]
fn test_init_creates_directory_safely() {
    let temp_dir = env::temp_dir().join(format!("rgit_test_{}", line!()));
    fs::create_dir_all(&temp_dir).unwrap();

    // We pass a dummy vector to capture output if we care to assert against it,
    // or just to satisfy the function signature.
    let mut output_sink = Vec::new();
    let result = execute_command(&Command::Init, temp_dir.clone(), &mut output_sink);

    assert!(result.is_ok());
    assert!(
        temp_dir.join(".rgit").exists(),
        "The .rgit directory should have been created!"
    );

    fs::remove_dir_all(temp_dir).unwrap();
}
