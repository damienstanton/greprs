extern crate greprs;

use std::env;
use std::process;
use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Bad args: {}", err);
        process::exit(1);
    });

    if let Err(e) = greprs::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}