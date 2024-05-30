use std::io::{self, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.

    // Uncomment this block to pass the first stage

    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        println!("{}: command not found", input.trim())
    }
}
