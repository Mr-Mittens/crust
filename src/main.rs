use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    loop {
        print!(":: ➜ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error occurred!");

        let input = input.trim();
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        let child = Command::new(command)
            .args(&args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn();

        match child {
            Ok(mut child) => {
                child.wait().expect("Command failed to execute");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
