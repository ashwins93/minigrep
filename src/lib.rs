use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results;

    if !config.ignore_case {
        results = search(&config.query, &contents);
    } else {
        results = search_case_insensitive(&config.query, &contents);
    }

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = Regex::new(query).unwrap();
    let mut results = Vec::new();
    for line in contents.lines() {
        if query.is_match(line) {
            results.push(line)
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = format!("{}{}", "(?i)", query);
    let query = Regex::new(&query).unwrap();

    let mut results = Vec::new();
    for line in contents.lines() {
        if query.is_match(line) {
            results.push(line);
        }
    }

    results
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Duct.
Pick three.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }

    #[test]
    fn regex_match() {
        let query = "^sa\\w+";
        let contents = "\
Rust:
safe, fast, productive.
Duct.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
