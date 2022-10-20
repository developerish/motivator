use clap::Parser;
use std::process;

use motivator::Config;

fn main() {
    let config = Config::parse();

    if let Err(e) = motivator::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
