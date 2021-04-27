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
mod profile;

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
    let config_path = resolve_home_dir(
        matches.value_of("config").expect("Failed to read config path argument")
    );
    let config_file_path = format!("{}/config", config_path);
    // Create path for config if it doesn't exist
    if !Path::new(&config_path).exists() {
        fs::create_dir_all(&config_path).expect("Failed to create path to configuration directory.");
    }
    // Load configuration file
    let mut config_file = match config_file::load_config_file(&config_file_path) {
        Ok(config_file) => config_file,
        Err(e) => panic!("Failed to load configuration file: {}", e),
    };
    // Get path to current profile
    let profiles_path = format!("{}/profiles", config_path);
    let current_profile_file_path = format!("{}/{}", &profiles_path, &config_file.current_profile);
    let current_profile_file = profile::load_profile_file(&current_profile_file_path).expect("Failed to load current profile!");
    // Execute the given subcommand
    match matches.subcommand_name() {
        Some("cache") => {
            println!("{}", current_profile_file.data_path);
        }
        Some("config") => {
            // Implement this later
            exit(1);
        }
        Some("profile") => {
            // Check the given subcommand
            let subcommand_matches = matches.subcommand_matches("profile").unwrap();
            // Select a given profile
            match subcommand_matches.subcommand_name() {
                Some("list") | Some("ls") => {
                    let paths = fs::read_dir(profiles_path).unwrap();
                    println!("Available profiles:");
                    for path in paths {
                        println!("Profile: {:?}", path.unwrap().file_name());
                    }
                }
                Some("create") => {
                    let subsubcommand_matches = subcommand_matches.subcommand_matches("create").unwrap();
                    let new_profile_name = subsubcommand_matches.value_of("name").expect("Error reading name of new profile.");
                    let _new_profile = profile::load_profile_file(&format!("{}/{}", profiles_path, new_profile_name));
                    println!("Created profile {}", new_profile_name);
                }
                _ => {
                    println!("Command missing! Try with -h for more info.");
                    exit(1);
                }
            }
            // let new_profile = subcommand_matches.value_of("profile").expect("Failed to read new profile from command");
            // Test that the profile exists

            // config_file.current_profile = new_profile.to_string();
        }
        _ => {
            println!("Command missing! Try with -h for more info.");
            exit(1);
        }
    }
}
