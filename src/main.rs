#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env::{self, current_dir}, future, path::{Path, PathBuf}, process::{self, Command, Stdio}};
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
        let current_dir = env::current_dir();
        if current_dir.is_err() {
            println!("$ ");
        } else {
            print!("$ {}>", current_dir.unwrap().display());
        }
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
                let exit_code = inputs[1];
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
                    "exit" | "echo" | "type" | "pwd" | "cd" => {
                        println!("{typed_command} is a shell builtin")
                    },
                    _ => {
                        let target_path = find_it(typed_command);
                        if target_path.is_none() {
                            println!("{typed_command}: not found");
                        } else {
                            println!("{}", target_path.unwrap().display());
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
            },

            "cd" => {
                let _path = &inputs[1..].join(" ");
                // * with a relative path we can use grab the current dir and join it with the given one
                if _path.starts_with("C:") {
                    let path = PathBuf::new().join(_path);
                    match env::set_current_dir(&path) {
                        Err(err) => println!("Failed to set current directory: {}", err),
                        _ => {},
                    }
                } else if _path.starts_with(".") {
                    input = input.replace("cd ", "");
                    let mut path_cpm = input.split("\\").collect::<Vec<&str>>();
                    let main_path =  env::current_dir();
                    match main_path {
                        Err(err) => println!("issue reading the current active path {:#?}", err),
                        Ok(path) => {
                            let main_path = path.to_string_lossy().to_string();
                            let mut current_dir = main_path.split("\\").collect::<Vec<&str>>();
                            loop {
                                if !path_cpm.contains(&".") && !path_cpm.contains(&"..") {
                                    break;
                                }
                                for i in 0..path_cpm.len() - 1 {
                                    match path_cpm[i] {
                                        "." => {
                                            path_cpm.remove(i);
                                        }
                                        ".." => {
                                            path_cpm.remove(i);
                                            if !current_dir.is_empty() {
                                                current_dir.pop();
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }

                            let relative_path = path_cpm.join("\\");
                            current_dir.push(&relative_path);
                            let path = PathBuf::new().join(current_dir.join("\\").trim());
                            match env::set_current_dir(&path) {
                                Err(err) => println!("Failed to set current directory: {}", err),
                                _ => {},
                            }
                        }
                    }
                } else {
                    println!("cd: {_path} : No such file or directory");
                }

            }

            _ => {
                let target_path = find_it(&command);
                if target_path.is_none() {
                    println!("{}: not found", command);
                } else {
                    let output = Command::new   (command)
                        .args(&inputs[1..])
                        .stdout(Stdio::piped())
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