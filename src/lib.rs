use std::error::Error;
use std::fs;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}")
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String= args[2].clone();

        Ok(Config {query, file_path})
    }
}