use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn query(&self) -> &str {
        self.query.as_ref()
    }
    pub fn file_path(&self) -> &str {
        self.file_path.as_ref()
    }
    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }

    // 通常 new 函数永远不会失败，如果要包含可能导致失败的逻辑，建议使用其它函数名
    pub fn build(args: &[String]) -> Result<Self, &str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path())?;

    let results = if config.ignore_case() {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";

    #[test]
    fn case_sensitive() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
    }

    fn case_insensitive() {
        let query = "rUsT";
        assert_eq!(
            vec!["Rust", "Trust me."],
            search_case_insensitive(query, CONTENTS)
        );
    }
}
