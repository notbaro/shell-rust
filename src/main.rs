use std::{
    io::{self, Write},
    process::exit,
};

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

        let args = input.split_whitespace().collect::<Vec<&str>>();

        match args[0] {
            "exit" => exit(args[1].parse::<i32>().unwrap()),
            "echo" => println!("{}", args[1..].join(" ")),
            _ => println!("{}: command not found", input),
        }
    }
}
