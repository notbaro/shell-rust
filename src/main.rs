use std::{
    env,
    io::{self, Write},
    path::Path,
    process::exit,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.

    // Uncomment this block to pass the first stage

    let stdin = io::stdin();
    let builtins = ["exit", "echo", "type"];
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
            "type" => {
                if builtins.contains(&args[1]) {
                    println!("{} is a shell builtin", args[1]);
                } else {
                    let path = env::var("PATH").unwrap();
                    let paths: Vec<&str> = path.split(':').collect();
                    let mut found = false;

                    for p in paths {
                        let mut full_path = Path::new(p).join(args[1]);
                        full_path.set_extension("");
                        if full_path.exists() {
                            println!("{} is {}", args[1], full_path.display());
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        println!("{}: not found", args[1]);
                    }
                }
            }
            _ => println!("{}: command not found", input),
        }
    }
}
