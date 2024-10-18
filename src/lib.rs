use std::{env, error::Error, fs};
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("should be able to read file");
    let results = if config.ignore_case {
        search_insensentive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for result in results {
        println!("{result}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.contains(query))
        .collect::<Vec<&str>>()
}

pub fn search_insensentive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_return_case_sensitive_results() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn it_should_return_case_insensitive_results() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            search_insensentive(query, contents),
            vec!["Rust:", "Trust me."],
        );
    }
}
