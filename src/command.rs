use std::{
    env,
    path::{Path, PathBuf},
    process,
};
pub enum Command<'a> {
    Exit(i32),
    Echo(Vec<&'a str>),
    Type(&'a str),
    NotBuiltin(&'a str, Vec<&'a str>),
}
impl<'a> Command<'a> {
    pub fn execute(self) {
        match self {
            Command::Exit(code) => {
                std::process::exit(code);
            }
            Command::Echo(args) => {
                println!("{}", args[1..].join(" "));
            }
            Command::Type(cmd) => match cmd {
                "exit" | "echo" | "type" => {
                    println!("{} is a shell builtin", cmd)
                }
                _ => {
                    if let Some(bin_path) = find_in_path(cmd) {
                        println!("{cmd} is {path}", path = bin_path.display());
                    } else {
                        println!("{cmd}: not found");
                    }
                }
            },
            Command::NotBuiltin(cmd, args) => {
                if let Some(bin_path) = find_in_path(cmd) {
                    let mut process = process::Command::new(bin_path)
                        .args(args)
                        .stdout(std::io::stdout())
                        .spawn()
                        .unwrap();
                    process.wait().unwrap();
                } else {
                    println!("{cmd}: command not found")
                }
            }
        }
    }
}
pub fn into_command(raw_args: &str) -> Command {
    let parsed_args: Vec<&str> = raw_args.split_whitespace().collect::<Vec<&str>>();
    let cmd = parsed_args[0];
    match cmd {
        "exit" => Command::Exit(parsed_args[1].parse::<i32>().unwrap()),
        "echo" => Command::Echo(parsed_args),
        "type" => Command::Type(parsed_args[1]),
        _ => Command::NotBuiltin(cmd, parsed_args[1..].to_vec()),
    }
}
fn find_in_path(bin_name: &str) -> Option<PathBuf> {
    let raw_path = env::var("PATH").unwrap_or_default();
    let paths = raw_path.split(":");
    for path in paths {
        let cmd_path = Path::new(path).join(bin_name);
        if cmd_path.exists() {
            return Some(cmd_path);
        }
    }
    None
}
