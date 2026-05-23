use std::env;

#[derive(Debug)]
struct CliConfig {
    postional_args: Vec<String>,
}

fn main() {
    // Initialize program configuration
    let mut config = CliConfig {
        postional_args: Vec::new(),
    };

    // Turn arguments into an iterator
    let mut args_iter = env::args();
    args_iter.next();

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "init" => {
                println!("Initializing repository...")
            }

            "-h" | "--help" => {
                println!("Rgit is mock implementation of git in Rust");
                println!("Usage [FLAGS] [ARGS]")
            }

            // Save everything else as regular positional arguments
            positional => config.postional_args.push(positional.to_string()),
        }
    }
}
