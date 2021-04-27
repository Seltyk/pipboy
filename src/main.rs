// Copyright (C) 2021 Aayla Semyonova
// 
// This file is part of pipboy.
// 
// pipboy is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// pipboy is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with pipboy.  If not, see <http://www.gnu.org/licenses/>.

mod config_file;

#[macro_use]
extern crate clap;
use clap::App;
use std::process::exit;
use std::env;
use substring::Substring;
use std::fs;
use std::path::Path;

fn resolve_home_dir(path: &str) -> String {
    if path == "~" || path.substring(0, 2) == "~/" {
        // Get the home directory from environment variables
        let home_directory = env::var("HOME").expect("HOME is undefined in environment variables");
        let result = str::replace(path, "~", &home_directory);
        result
    } else {
        path.to_string()
    }
}

fn main() {
    // Load CLI arguments with clap
    let yaml = load_yaml!("arguments.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    // Get config path
    let config_file_path = resolve_home_dir(
        matches.value_of("config").expect("Failed to read config path argument")
    );
    let config_path = Path::new(&config_file_path).parent().expect("This error shouldn't be possible.");
    // Create path for config if it doesn't exist
    fs::create_dir_all(config_path).expect("Failed to create path to configuration directory.");
    // Load configuration file
    let config_file = match config_file::load_config_file(&config_file_path) {
        Ok(config_file) => config_file,
        Err(e) => panic!("Failed to load configuration file: {}", e),
    };
    // Execute the given subcommand
    match matches.subcommand_name() {
        Some("cache") => {
            println!("{}", config_file.data_path)
        }
        Some("config") => {
            // Implement this later
            exit(1);
        }
        Some("profile") => {
            // Implement this later
            exit(1);
        }
        _ => {
            println!("Command missing! Try with -h for more info.");
            exit(1);
        }
    }
}
