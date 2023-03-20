use std::error::Error;
use std::{env, fs, process};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? 遇到错误时会将错误返回给调用者
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 1 {
            return Err("not enough arguments!");
        }
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        let case_sensitive = match args.next() {
            Some(_) => false,
            None => env::var("CASE_INSENSITIVE").is_err()
        };

        /*if args.len() == 4 && case_insensitive == "case_insensitive" {
            case_sensitive = false;
        } else {
            case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        }*/

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results*/

    contents.lines().filter(|v| {
        v.contains(query)
    }).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results*/
    contents.lines().filter(|v| {
        v.to_lowercase().contains(&query.to_lowercase())
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
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
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents))
    }
}