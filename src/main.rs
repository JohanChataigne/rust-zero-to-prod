use std::{env, process};
use zero2prod::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1)
    });

    if let Err(e) = zero2prod::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
