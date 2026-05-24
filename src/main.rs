use std::env;

fn main() {
    let config = rgit::parse_args(env::args(), std::io::stdout());

    let current_dir = env::current_dir().unwrap_or_default();

    if let Err(e) = rgit::execute_command(&config.command, current_dir) {
        eprintln!("Error executing command: {}", e);
    }
}
