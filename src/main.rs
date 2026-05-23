// src/main.rs
use std::env;

fn main() {
    // env::args() satisfies IntoIterator, io::stdout() satisfies Write
    let config = rgit::parse_args(env::args(), std::io::stdout());
    println!("Config successfully loaded: {:?}", config);
}
