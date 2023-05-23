use std::env;
use std::process;
use tinygrep::Config;

// The tinygrep entry point
fn main() {
    // Capture args from the program call
    let args: Vec<String> = env::args().collect();

    // Loads the file's content
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Print args
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Calls the config run()
    if let Err(e) = tinygrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
