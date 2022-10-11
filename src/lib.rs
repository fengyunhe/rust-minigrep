use std::error::Error;
use std::fs::read_to_string;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(config.file).expect("file error");
    println!("Text: \n{}", contents);
    Ok(())
}

pub struct Config {
    pub keywords: String,
    pub file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let keywords = args[1].clone();
        let file = args[2].clone();
        Ok(Config { keywords, file })
    }
}
