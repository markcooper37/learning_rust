// Creating a version of the command line tool grep
// using chapter 12 of 'The Rust Programming Language'

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}