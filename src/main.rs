use std::env;

fn main() {
    let config = rgit::parse_args(env::args());
    println!("Config successfully loaded: {:?}", config);
}
