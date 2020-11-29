use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for l in result {
        println!("line {}: {}", l.number, l.line);
    }

    Ok(())
}

#[derive(Debug)]
pub struct Line {
    pub line: String,
    pub number: u32,
}

impl Line {
    pub fn new(line: String, number: u32) -> Line {
        Line { line, number }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.line == other.line && self.number == other.number
    }
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<Line> {
    let mut results: Vec<Line> = Vec::new();
    let mut number: u32 = 0;

    for line in contents.lines() {
        number = number + 1;
        results.push(Line::new(line.to_string(), number));
    }

    results
        .into_iter()
        .filter(|l| l.line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<Line> {
    let query = query.to_lowercase();
    let mut results: Vec<Line> = Vec::new();
    let mut number: u32 = 0;

    for line in contents.lines() {
        number = number + 1;
        results.push(Line::new(line.to_string(), number));
    }

    results
        .into_iter()
        .filter(|l| l.line.to_lowercase().contains(&query))
        .collect()
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
Pick three.
Duct tape.";

        assert_eq!(
            vec![Line::new("safe, fast, productive.".to_string(), 2)],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![
                Line::new("Rust:".to_string(), 1),
                Line::new("Trust me.".to_string(), 4)
            ],
            search_case_insensitive(query, contents)
        );
    }
}
