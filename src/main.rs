use std::env;

// The tinygrep entry point
fn main() {
    // Capture args from the program call
    let args: Vec<String> = env::args().collect();

    // The query arg
    let query = &args[1];

    // The file path
    let file_path = &args[2];

    // Print args
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
