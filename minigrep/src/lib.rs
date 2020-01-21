use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query,  filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    //println!("with text:\n{}", contents);
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query :&str, contents: &'a str) ->Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }

    result
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
Pickthree.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents)
        )
    }
}
