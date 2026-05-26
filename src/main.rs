use std::env;
use std::io;

fn main() {
    // Parsing is clean and independent of stdout
    let config = rgit::parse_args(env::args());

    let current_dir = env::current_dir().unwrap_or_default();

    // Side-effects and execution happen here
    if let Err(e) = rgit::execute_command(&config.command, current_dir, io::stdout()) {
        eprintln!("Error executing command: {}", e);
    }
}
