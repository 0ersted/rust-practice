use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new (mut args: std::env::Args) -> Result<Config, &'static str> {
        // first argument is about the program info. skip
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None =>return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query,  filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    //println!("with text:\n{}", contents);
    /*let result = if config.case_sensitive {
        search(&config.query, &contents);
    } else {
        search_case_insensitive(&config.query, &contents);
    };*/
    
    let mut result;

    if config.case_sensitive {
        result = search(&config.query, &contents);
    } else {
        result = search_case_insensitive(&config.query, &contents);
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query :&str, contents: &'a str) ->Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query :&str, contents: &'a str) ->Vec<&'a str> {
    let _query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&_query) {
            result.push(line)
        }
    }

    result
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
Pickthree.
DuTi boi.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents)
        )
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
