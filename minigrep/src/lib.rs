use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let lines: Vec<Line> = lines_to_vec_lines(&contents);

    let result = if config.case_sensitive {
        search(&config.query, lines)
    } else {
        search_case_insensitive(&config.query, lines)
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

pub fn search(query: &str, contents: Vec<Line>) -> Vec<Line> {
    contents
        .into_iter()
        .filter(|l| l.line.contains(query))
        .collect()
}

pub fn search_case_insensitive(query: &str, contents: Vec<Line>) -> Vec<Line> {
    let query = query.to_lowercase();
    contents
        .into_iter()
        .filter(|l| l.line.to_lowercase().contains(&query))
        .collect()
}

fn lines_to_vec_lines(contents: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut number: u32 = 0;
    for l in contents.lines() {
        number = number + 1;
        lines.push(Line::new(l.to_string(), number));
    }

    lines
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
        let lines: Vec<Line> = lines_to_vec_lines(contents);

        assert_eq!(
            vec![Line::new("safe, fast, productive.".to_string(), 2)],
            search(query, lines)
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
        let lines: Vec<Line> = lines_to_vec_lines(contents);
        assert_eq!(
            vec![
                Line::new("Rust:".to_string(), 1),
                Line::new("Trust me.".to_string(), 4)
            ],
            search_case_insensitive(query, lines)
        );
    }
}
