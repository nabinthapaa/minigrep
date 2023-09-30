use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguements");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive{
        case_sensitive_search(&config.query, &contents)
    }else{
        case_insensitive_search(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

pub fn case_sensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line.trim());
        }
    }
    results
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line.trim());
        }
    }
    results
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape";
        assert_eq!(vec!["safe, fast, productive."],case_sensitive_search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUSt";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],case_insensitive_search(query, contents));
    }
}
