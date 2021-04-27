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

#[macro_use]
extern crate clap;
use clap::App;
use std::process::exit;
use std::env;
use substring::Substring;

fn resolve_home_dir(path: &str) -> String {
    if path.substring(0, 1) == "~" {
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
    let config_path = resolve_home_dir(
        matches.value_of("config").expect("Failed to read config path argument")
    );
    // Create config file if it doesn't exist
    


    // Execute the given subcommand
    match matches.subcommand_name() {
        Some("cache") => {
            println!("{}", config_path);
        }
        _ => {
            println!("Command missing! Try with -h for more info.");
            exit(1);
        }
    }
}
