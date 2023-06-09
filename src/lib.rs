use std::error::Error;
use std::fs;

// Runs the logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Reads the file contents
    let contents = fs::read_to_string(config.file_path)?;

    // Prints file contents
    println!("With text:\n{contents}");

    Ok(())
}

// A Config struct
pub struct Config {
    pub query: String,
    pub file_path: String,
}

// Parses arguments
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Some error handling
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        // Reads each args
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Builds a Config object to return
        Ok(Config { query, file_path })
    }
}

// Performs the search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
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
}
