use std::{fs, io};
use std::io::{stdout, Write};
use std::path::Path;
use std::process::Command;
use colored::*;
use std::env;

fn main() {
    // Set working dir to %USERPROFILE%
    if let Ok(home) = env::var("USERPROFILE") {
        env::set_current_dir(&home).expect("Failed to set current dir to USERPROFILE");
    }

    println!("{}", "Welcome To ArtiFIleMan".bold().cyan());

    loop {
        print!("{}", format!("{}> ", env::current_dir().unwrap().display()).bright_green());
        stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("{}", "Failed to read input.".red());
            continue;
        }

        let input = input.trim();
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match command {
            "cp" => {
                let (src, dst) = read_two_args(&args, "path to copy from", "destination path");
                if !Path::new(&src).exists() {
                    eprintln!("{}", "Source file doesn't exist.".red());
                } else if let Err(e) = fs::copy(&src, &dst) {
                    eprintln!("{} {}", "Failed to copy file:".red(), e);
                } else {
                    println!("{}", "File copied successfully.".green());
                }
            }

            "rm" => {
                let file = read_one_arg(&args, "path to remove");
                if let Err(e) = fs::remove_file(&file) {
                    eprintln!("{} {}", "Failed to remove file:".red(), e);
                } else {
                    println!("{}", "File removed.".green());
                }
            }

            "mv" | "cut" => {
                let (src, dst) = read_two_args(&args, "path to move/cut", "destination path");
                if !Path::new(&src).exists() {
                    eprintln!("{}", "File doesn't exist.".red());
                } else if let Err(e) = fs::rename(&src, &dst) {
                    eprintln!("{} {}", "Failed to move file:".red(), e);
                } else {
                    println!("{}", "File moved.".green());
                }
            }

            "ren" => {
                let (src, dst) = read_two_args(&args, "file to rename", "new name");
                if let Err(e) = fs::rename(&src, &dst) {
                    eprintln!("{} {}", "Failed to rename file:".red(), e);
                } else {
                    println!("{}", "File renamed.".green());
                }
            }

            "mkdir" => {
                let dir = read_one_arg(&args, "directory to create");
                if let Err(e) = fs::create_dir_all(&dir) {
                    eprintln!("{} {}", "Failed to create directory:".red(), e);
                } else {
                    println!("{}", "Directory created.".green());
                }
            }

            "touch" => {
                let file = read_one_arg(&args, "filename to touch");
                if let Err(e) = fs::OpenOptions::new().create(true).write(true).open(&file) {
                    eprintln!("{} {}", "Failed to create file:".red(), e);
                } else {
                    println!("{}", "File touched.".green());
                }
            }

            "chmod" => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let (perm_str, file) = read_two_args(&args, "permission (e.g. 755)", "file");
                    if let Ok(mode) = u32::from_str_radix(&perm_str, 8) {
                        if let Err(e) = fs::set_permissions(&file, fs::Permissions::from_mode(mode)) {
                            eprintln!("{} {}", "Failed to set permissions:".red(), e);
                        } else {
                            println!("{}", "Permissions updated.".green());
                        }
                    } else {
                        eprintln!("{}", "Invalid permission format. Use octal (e.g. 755)".red());
                    }
                }

                #[cfg(windows)]
                {
                    eprintln!("{}", "chmod is not supported on Windows.".yellow());
                }
            }

            "cd" => {
                let dir = read_one_arg(&args, "directory to change to");
                if let Err(e) = env::set_current_dir(&dir) {
                    eprintln!("{} {}", "Failed to change directory:".red(), e);
                }
            }

            "ls" | "dir" => {
                match fs::read_dir(env::current_dir().unwrap()) {
                    Ok(entries) => {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                println!("{}", entry.file_name().to_string_lossy().white());
                            }
                        }
                    }
                    Err(e) => eprintln!("{} {}", "Failed to list directory:".red(), e),
                }
            }

            "prop" => {
                let file = read_one_arg(&args, "file/folder for properties");
                #[cfg(windows)]
                {
                    if let Err(e) = Command::new("cmd")
                        .args(["/C", "explorer", "/select,", &file])
                        .spawn()
                    {
                        eprintln!("{} {}", "Failed to open properties:".red(), e);
                    }
                }

                #[cfg(not(windows))]
                {
                    eprintln!("{}", "prop is not implemented for non-Windows systems.".yellow());
                }
            }

            "exit" | "quit" => {
                println!("{}", "Exiting ArtiFIleMan. Peace out!".bright_blue());
                break;
            }

            "" => continue,

            _ => {
                println!("{}", "Unknown command.".red());
                println!("{}", "Available: cp, rm, mv, ren, mkdir, touch, chmod, cd, ls/dir, cut, prop, exit".yellow());
            }
        }
    }
}

fn read_one_arg(args: &[&str], prompt: &str) -> String {
    if args.is_empty() {
        print!("{} ", prompt.yellow());
        stdout().flush().unwrap();
        read_trimmed_line()
    } else {
        args[0].to_string()
    }
}

fn read_two_args(args: &[&str], prompt1: &str, prompt2: &str) -> (String, String) {
    let arg1 = if args.len() >= 1 {
        args[0].to_string()
    } else {
        print!("{} ", prompt1.yellow());
        stdout().flush().unwrap();
        read_trimmed_line()
    };

    let arg2 = if args.len() >= 2 {
        args[1].to_string()
    } else {
        print!("{} ", prompt2.yellow());
        stdout().flush().unwrap();
        read_trimmed_line()
    };

    (arg1, arg2)
}

fn read_trimmed_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
    
}
