use std::{env::args, process::exit};

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments :{}", err);
        exit(1)
    });
    println!(
        "Search keywords {} from file {}",
        config.keywords, config.file
    );
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        exit(1)
    };
}
