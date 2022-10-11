use std::{env, process};

use motivator::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argumens: {err}");
        process::exit(1);
    });

    if let Err(e) = motivator::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
