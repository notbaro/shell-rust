use std::{io::{self, Write}, process::exit};

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
        let input = input.trim();
        
        let tokens = split_command(input);
        match tokens[..] {
            ["exit", exit_code] => exit(exit_code.parse::<i32>().unwrap()),
            _ => println!("{}: command not found", input.trim())
        }

    }
}

fn split_command(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}
