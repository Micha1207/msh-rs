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
//! Main file.

use colored::*;

mod cmds;
mod helpers;

fn main() {
    println!(
        "\n{}",
        "Welcome to MSH Rust by Micha1207. (C) 2026 Micha1207."
            .blue()
            .bold()
    );
    println!(
        "{}",
        "MSH Rust - Micha1207's Shell Program written in Rust Language.".red()
    );
    println!("This program comes with WITHOUT ANY WARRANTY");
    println!("and is free software, licensed under GNU GPL v3.");
    println!(
        "To view all available commands type: {}",
        "help".red().bold()
    );

    loop {
        let input = helpers::prompt();
        let arg: Vec<String> = match shell_words::split(&input) {
            Ok(parsed_args) => parsed_args,
            Err(_) => {
                println!("{}", "msh-rs: mismatched quotes.".red().bold());
                continue;
            }
        };

        if arg.is_empty() {
            continue;
        }

        match arg[0].as_str() {
            "exit" => break,
            "cat" => {
                let path = arg
                    .iter()
                    .skip(1)
                    .find(|o| !o.starts_with('-'))
                    .map(|s| s.as_str());

                let show_line_num: bool = arg.iter().skip(1).any(|o| o == "-n" || o == "--num");
                let show_line_ends: bool = arg.iter().skip(1).any(|o| o == "-E" || o == "--ends");

		if let Some(p) = path {
		    if let Err(e) = cmds::cat(p, show_line_num, show_line_ends) {
			println!("cat: {}", e);
                    }
		} else {
		    println!("cat: missing filename.");
		}
            }
            "mkdir" => {
                if let Some(name) = arg.get(1) {
                    if let Err(e) = cmds::mkdir(name) {
                        println!("mkdir: {}", e);
                    }
                } else {
                    println!("mkdir: no name provided.");
                }
            }
            "ls" => {
                let path = arg
                    .iter()
                    .skip(1)
                    .find(|o| !o.starts_with('-'))
                    .map(|s| s.as_str())
                    .unwrap_or(".");

                let show_hidden = arg.iter().skip(1).any(|o| o == "-a" || o == "--all");
                let show_type = arg.iter().skip(1).any(|o| o == "-t" || o == "--type");

                if let Err(e) = cmds::ls(path, show_hidden, show_type) {
                    println!("ls: {}", e);
                }
            }
            "pwd" => {
                if let Err(e) = cmds::pwd() {
                    println!("pwd: {}", e);
                }
            }
            "touch" => {
                if let Some(file) = arg.get(1) {
                    if let Err(e) = cmds::touch(file) {
                        println!("touch: {}", e);
                    }
                } else {
                    println!("touch: no file provided.");
                }
            }
            "rm" => {
                if let Some(file) = arg.get(1) {
                    if let Err(e) = cmds::remove(file) {
                        println!("rm: {}", e);
                    }
                } else {
                    println!("rm: no file provided.");
                }
            }
            "echo" => {
                if let Err(e) = cmds::echo(&arg) {
                    println!("echo: {}", e);
                }
            }
            "cd" => {
                let path: String = if let Some(path) = arg.get(1) {
                    if path == "~" {
                        helpers::get_home_dir()
                    } else {
                        path.to_string()
                    }
                } else {
                    helpers::get_home_dir()
                };

                if let Err(e) = cmds::cd(&path) {
                    println!("cd: {}.", e);
                }
            }
            "clear" | "cls" => cmds::clear(),
            "help" => {
                if let Some(cmd) = arg.get(1) {
                    if let Err(e) = cmds::help(cmd) {
                        println!("help: {}", e);
                    }
                } else {
                    if let Err(e) = cmds::help("msh") {
                        println!("help: {}", e);
                    }
                }
            }
            "" => {}
            _ => {
                if let Err(e) = cmds::run(&arg) {
                    println!("run: {}", e);
                }
            }
        }
    }
}
