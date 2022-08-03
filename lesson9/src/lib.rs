use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("{}", contents);

    for line in search(&config.query, &contents) {
        println!("line: {}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("no filename"),
        };
        Ok(Config { query, filename })
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "/
        asdasd

duct adad
        ";

        assert_eq!(vec!["duct adad"], search(query, contents))
    }
}
