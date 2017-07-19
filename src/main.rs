use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = parse_config(&args);
    
    let mut f = File::open(&config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("unable to read file");
    
    println!("Looking for {} in file {} with text:\n {}", config.query, config.filename, contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    // remember to use more efficient methods than `clone` moving forward...
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {
        query: query,
        filename: filename,
    }
}