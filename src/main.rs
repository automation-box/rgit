use std::env;
use std::io;

fn main() {
    // Parsing is clean and independent of stdout
    let config = rgit::parse_args(env::args());

    // Side-effects and execution happen here
    if let Err(e) = rgit::execute_command(&config.command, io::stdout()) {
        eprintln!("Error executing command: {}", e);
    }
}
