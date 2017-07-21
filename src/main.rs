extern crate greprs;

use std::env;
use std::process;
use greprs::Config;
use greprs::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(e) = run(config) {
        process::exit(1);
    }
}