use std::error::Error;
use std::fs;
use std::vec::Vec;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/*
fn parse_config(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!(
            "Not enough arguments.\nUsage: {:?} <pattern> <file_path",
            args[0]
        );
    }

    let query = args[1].clone();
    let file_path = args[2].clone();

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Config { query, file_path, ignore_case }
}
*/

// improvement (or just a variant) over above
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // the '?' operator in the end is key, it can return Error (we studied this in an earlier chapter)
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // println!("With text:\n{contents}");
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    /*
    commenting this to use iterator adaptor way
    let mut v = Vec::<&str>::new();
    for line in contents.lines() {
        if line.contains(query) {
            v.push(line);
        }
    }
    v
    */

    // iterator adaptor approach prefered in rust over looping (chapter 13.3)
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    /*
    commenting this to use iterator adaptor way
    let mut v = Vec::<&str>::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            v.push(line);
        }
    }
    v
    */

    // iterator adaptor approach prefered in rust over looping (chapter 13.3)
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
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
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            search_case_insensitive(query, contents)
        );
    }
}
