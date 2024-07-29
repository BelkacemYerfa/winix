#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::{Path, PathBuf}, process::{self, Command}};
use std::borrow::Cow;

mod test;

fn main() {
     //Uncomment this block to pass the first stage

    // * this will insure the cross platform compatibility
    #[cfg(not(target_os = "windows"))]
    fn enhance_exe_name(exe_name: &Path) -> Cow<Path> {
        exe_name.into()
    }

    #[cfg(target_os = "windows")]
    fn enhance_exe_name(exe_name: &Path) -> Cow<Path> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        let raw_input: Vec<_> = exe_name.as_os_str().encode_wide().collect();
        let raw_extension: Vec<_> = OsStr::new(".exe").encode_wide().collect();

        if raw_input.ends_with(&raw_extension) {
            exe_name.into()
        } else {
            let mut with_exe = exe_name.as_os_str().to_owned();
            with_exe.push(".exe");
            PathBuf::from(with_exe).into()
        }
    }

    fn find_it<P>(exe_name: P) -> Option<PathBuf>
        where P: AsRef<Path>,
        {
            let exe_name = enhance_exe_name(exe_name.as_ref());
            env::var_os("PATH").and_then(|paths| {
                env::split_paths(&paths).filter_map(|dir| {
                    let full_path = dir.join(&exe_name);
                    if full_path.is_file() {
                        Some(full_path)
                    } else {
                        None
                    }
                }).next()
            })
        }

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let inputs = input.trim().split(" ").filter(
            |inp| inp.len() > 0
        ).collect::<Vec<&str>>();

        if input.len() == 0 {
            continue;
        }

        let command = inputs[0].to_lowercase();

        match command.as_str() {
            "exit" => {
                let exit_code = &inputs[1];
                process::exit(
                    exit_code.parse::<i32>().unwrap()
                )
            }
            "echo" => {
                let echo_data = &inputs[1..].join(" ");
                println!("{}", echo_data);
            }
            "type" => {
                let typed_command = inputs[1];

                match typed_command {
                    "exit" | "echo" | "type" | "pwd" => {
                        println!("{typed_command} is a shell builtin")
                    },
                    _ => {
                        let target_path = find_it(typed_command);
                        if target_path.is_none() {
                            println!("{typed_command}: not found");
                        } else {
                            println!("{:?}", target_path.unwrap());
                        }
                    }
                }
            },

            "pwd" => {
                let current_dir = env::current_dir();
                if current_dir.is_err() {
                    println!("There was an issue printing the current working dir");
                } else {
                    println!("{}", current_dir.unwrap().display());
                }
            }

            _ => {
                let target_path = find_it(&command);
                if target_path.is_none() {
                    println!("{}: not found", command);
                } else {
                    let output = Command::new   (command)
                        .args(&inputs[1..])
                        .output()
                        .expect("there was an issue executing u're program");
                    if output.status.success() {
                        println!("{}", String::from_utf8(output.stdout).unwrap());
                    }
                }
            }
        }
    }
}