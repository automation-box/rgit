use std::env;

#[derive(Debug, PartialEq)]
pub struct CliConfig {
    pub positional_args: Vec<String>,
}

pub fn parse_args(args: env::Args) -> CliConfig {
    let mut config = CliConfig {
        positional_args: Vec::new(),
    };

    // Skip binary path
    let mut args_iter = args;
    args_iter.next();

    for arg in args_iter {
        match arg.as_str() {
            "init" => {
                println!("Initializing repository...");
            }
            "-h" | "--help" => {
                println!("Rgit is mock implementation of git in Rust");
                println!("Usage [FLAGS] [ARGS]");
            }
            // Save everything else as regular positional arguments
            positional => config.positional_args.push(positional.to_string()),
        }
    }

    config
}
