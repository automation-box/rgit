use std::env;

fn main() {
    let config = rgit::parse_args(env::args());

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,

        Err(err) => {
            eprintln!("Failed to determine current directory: {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = rgit::execute_command(&config.command, &current_dir, std::io::stdout()) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
