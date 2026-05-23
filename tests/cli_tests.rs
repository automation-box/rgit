use rgit::parse_args;

#[test]
fn test_init_command_behavior() {
    let args = vec!["rgit".to_string(), "init".to_string()];
    let mut output_buffer = Vec::new();

    let config = parse_args(args, &mut output_buffer);

    // Verify stdout message contains expected trailing newline
    let output_string = String::from_utf8(output_buffer).unwrap();
    assert_eq!(output_string, "Initializing repository...\n");

    // Verify 'init' remains an executed action and is not swept into positional arguments
    assert_eq!(
        config.positional_args,
        Vec::<String>::new(),
        "Expected positional_args to be completely empty"
    );
}

#[test]
fn test_positional_arguments_are_captured() {
    let args = vec![
        "rgit".to_string(),
        "some_file.txt".to_string(),
        "another_file.rs".to_string(),
    ];
    let mut buffer = Vec::new();

    let config = parse_args(args, &mut buffer);

    // Direct structural comparison is cleaner and catches ordering bugs automatically
    assert_eq!(
        config.positional_args,
        vec!["some_file.txt".to_string(), "another_file.rs".to_string()]
    );
}
