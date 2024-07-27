#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

mod test;

fn main() {
     //Uncomment this block to pass the first stage


    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input = input.trim().to_string().to_lowercase();
        match input {
            input if input.starts_with("exit") => {
                let exit_code = input.split("exit").collect::<Vec<&str>>()[1].replace(" ", "");
                process::exit(
                    exit_code.parse::<i32>().unwrap()
                )
            }

            _ => {
                println!("{}: command not found" , input.trim());
            }
        }
    }
}