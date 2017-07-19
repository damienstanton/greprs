use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Bad args: {}", err);
        process::exit(1);
    });
    
    let mut f = File::open(&config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("unable to read file");
    
    println!("Looking for {} in file {} with text:\n {}", config.query, config.filename, contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("must be at least 3 args");
        }
        // remember to use more efficient methods than `clone` moving forward...
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}