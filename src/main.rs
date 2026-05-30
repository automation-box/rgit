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
        // eprintln!("Error: {}", err);
        match err {
            // For struct-like variants, use curly braces to get the fields by name
            rgit::RgitError::StorageFailure { path, source } => {
                eprintln!("Storage failed at {:?}", path);
                eprintln!("The OS error was {}", source);
            }
            // For tuple-like variants (like your transparent Io), use parentheses
            rgit::RgitError::Io(inner_io_error) => {
                eprintln!("A general IO error occurred: {}", inner_io_error);
            }
            // Handle other variants (like DirectoryAlreadyExists)
            _ => eprintln!("Another error occurred: {}", err),
        }
        std::process::exit(1);
    }
}
