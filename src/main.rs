extern crate greprs;

use std::env;
use std::process;
use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap();

    if let Err(e) = greprs::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}