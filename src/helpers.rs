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
//! Helper functions ("helpers").

use colored::*;
use std::io::{self, Write};

pub fn get_username() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| String::from("I have no name!"))
}

pub fn get_hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| String::from("My computer has no name!"))
}

pub fn get_home_dir() -> String {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| String::from("."))
}

pub fn prompt() -> String {
    let username = get_username();
    let hostname = get_hostname();
    let current_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let home_dir = std::path::PathBuf::from(get_home_dir());

    let display_path = if let Ok(stripped) = current_path.strip_prefix(&home_dir) {
        std::path::Path::new("~")
            .join(stripped)
            .display()
            .to_string()
    } else {
        current_path.display().to_string()
    };

    print!(
        "{}{}{}{}{} {}{}{} ",
        "[".blue().bold(),
        format!("{}", username).green().bold(),
        "@".red().bold(),
        format!("{}", hostname).green().bold(),
        ":".red().bold(),
        format!("{}", display_path).green(),
        "]".blue().bold(),
        "$".magenta().bold()
    );

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim_end().to_string()
}
