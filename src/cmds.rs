// MSH Rust - Micha1207's Shell Program written in Rust Language.
//
// Copyright (C) 2026 Micha1207
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! Builtin commands implementation.

use colored::*;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::Command;

pub fn cat(filename: &str, show_line_nums: bool, show_line_ends: bool) -> io::Result<()> {
    if filename.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing filename.",
        ));
    }

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for (index, line_read) in reader.lines().enumerate() {
        if let Ok(line) = line_read {
            let nums = if show_line_nums {
                format!("{:3}   ", format!("{}", index + 1).green())
            } else {
                String::new()
            };

            let ends = if show_line_ends {
                "$".red().to_string()
            } else {
                String::new()
            };

            println!("{}{}{}", nums, line, ends);
        }
    }

    Ok(())
}

pub fn cd(path: &str) -> io::Result<()> {
    if path.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Missing Path."));
    }

    std::env::set_current_dir(path)?;

    Ok(())
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn echo(args: &[String]) -> io::Result<()> {
    let elements: Vec<&str> = args.iter().skip(1).map(String::as_str).collect();

    println!("{}", elements.join(" "));

    Ok(())
}

pub fn help(cmd: &str) -> io::Result<()> {
    println!(
        "{}",
        "MSH Rust by Micha1207. (C) 2026 Micha1207".blue().bold()
    );
    println!(
        "{}",
        "MSH Rust - Micha1207's Shell Program written in Rust Language.".red()
    );
    println!("This program comes WITHOUT ANY WARRANTY");
    println!("and is free software, licensed under GNU GPL v3.");

    if cmd == "msh" {
        println!("{}", "Available commands:".red().bold());
        println!(
            "{}",
            " cat [FILE] [OPTION] - print content of file [FILE]\n \
              ls [OPT] [DIR]      - list objects in [DIR]\n \
              mkdir [DIR]         - make directory [DIR]\n \
              touch [FILE]        - make new file [FILE]\n \
              rm [OBJECT]         - remove [OBJECT] (all types)\n \
              echo [PHRASE]       - print [PHRASE] back\n \
              cd [DIR]            - change directory to [DIR]\n \
              cls, clear          - clear terminal window\n \
              help                - print this text\n \
              <command>           - run anything from PATH system variable\n \
              exit                - exit program"
                .yellow()
        );
        println!(
            "\nType: {} for other commands.",
            "help [COMMAND]".red().bold()
        );
    } else {
        println!("{}", format!("Help for: {}", cmd).red().bold());
        match cmd {
            "cat" => {
                println!("cat [FILE] [OPTION] - print content of file [FILE].");
                println!("[OPTION] can be:");
                println!("   -n or --nums     - print line numbers,");
                println!("   -E or --ends     - print $ on line ends.");
            }
            "ls" => {
                println!("ls [DIR] [OPTION]  - print content of directory [DIR].");
                println!(
                    "{} [DIR] on default is current working directory.",
                    "Warning:".red().bold()
                );
                println!("[OPTION] can be:");
                println!("   -a or --all     - show hidden files (beginning with a '.'),");
                println!("   -t or --type    - show type of objects:");
                println!("      'd' is a directory,");
                println!("      '-' is a file.");
            }
            "mkdir" => {
                println!("{}", "mkdir [DIR]         - make directory [DIR]".yellow());
            }
            "touch" => {
                println!("{}", "touch [FILE]        - make new file [FILE]".yellow());
            }
            "rm" => {
                println!(
                    "{}",
                    "rm [OBJECT]         - remove [OBJECT] (can be directory or file)".yellow()
                );
            }
            "echo" => {
                println!("{}", "echo [PHRASE]       - print [PHRASE] back".yellow());
            }
            "cd" => {
                println!(
                    "{}",
                    "cd [DIR]            - change directory to [DIR]".yellow()
                );
            }
            "cls" | "clear" => {
                println!("{}", "cls, clear          - clear terminal window".yellow());
            }
            "help" => {
                println!("{}", "help                - print this text".yellow());
            }
            "exit" => {
                println!("{}", "exit                - exit program".yellow());
            }
            _ => {
                println!(
                    "{}: command doesn't have definition in MSH Rust. Try {} --help",
                    cmd, cmd
                );
            }
        }

	println!("\nFor other commands type: {}", "help".red().bold());
    }

    Ok(())
}

pub fn mkdir(name: &str) -> io::Result<()> {
    if name.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Directory name cannot be empty.",
        ));
    }

    fs::create_dir(name)?;

    Ok(())
}

pub fn ls(input_path: &str, show_hidden: bool, show_type: bool) -> io::Result<()> {
    let objects = fs::read_dir(input_path)?;

    for object in objects {
        let path = object?.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if !show_hidden {
                if name.starts_with('.') {
                    continue;
                }
            }

            if show_type {
                if path.is_dir() {
                    println!("d {}", name.blue());
                } else {
                    println!("- {}", name);
                }
            } else {
                if path.is_dir() {
                    println!("{}", name.blue());
                } else {
                    println!("{}", name);
                }
            }
        }
    }

    Ok(())
}

pub fn pwd() -> io::Result<()> {
    let current_path = std::env::current_dir()?;

    println!("{}", current_path.display().to_string().green());

    Ok(())
}

pub fn remove(name: &str) -> io::Result<()> {
    if name.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing object name.",
        ));
    }

    let path = Path::new(name);

    if path.is_dir() {
        fs::remove_dir_all(&path)?;
    } else if path.exists() {
        fs::remove_file(&path)?;
    }

    Ok(())
}

pub fn run(arg: &[String]) -> io::Result<()> {
    if arg.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing command to run.",
        ));
    }

    let name = &arg[0];
    let args = &arg[1..];

    match Command::new(name).args(args).status() {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let local_path = Path::new(".").join(name);

            if local_path.exists() {
                Command::new(local_path).args(args).status().map(|_| ())?;
		Ok(())
            } else {
                println!("{}: unknown command.", arg[0]);
                Err(io::Error::new(
		    io::ErrorKind::NotFound,
		    format!("{}: unknown command.", name)
		))
            }
        }
        Err(e) => {
            Err(e)
        }
    }
}

pub fn touch(filename: &str) -> io::Result<()> {
    if filename.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing filename.",
        ));
    }

    fs::File::create(filename)?;

    Ok(())
}
