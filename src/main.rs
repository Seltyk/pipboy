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
mod archives;
mod cache;
mod remote;
mod mods;
mod file_ownership;

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
        let home_directory = env::var("HOME")
            .expect("HOME is undefined in environment variables");
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
    let verbose = matches.is_present("verbose");
    // Get config path
    let config_path = resolve_home_dir(
        matches.value_of("config")
            .expect("Failed to read config path argument")
    );
    // Create path for config if it doesn't exist
    if !Path::new(&config_path).exists() {
        fs::create_dir_all(&config_path)
            .expect("Failed to create path to configuration directory.");
    }
    // Load configuration file
    let config_file = match config_file::load_config_file(&config_path) {
        Ok(config_file) => config_file,
        Err(e) => panic!("Failed to load configuration file <- {}", e),
    };
    // Get path to current profile
    let profiles_path = format!("{}/profiles/", config_path);
    let current_profile_file_path = format!("{}/{}/profile", &profiles_path, &config_file.current_profile);
    let mut current_profile_file = profile::load_profile_file(&current_profile_file_path)
        .expect("Failed to load current profile!");
    // Execute the given subcommand
    match matches.subcommand_name() {
        Some("profile") => {
            // Check the given subcommand
            let subcommand_matches = matches.subcommand_matches("profile")
                .unwrap();
            // Select a given profile
            match subcommand_matches.subcommand_name() {
                Some("ls") => {
                    exit(match profile::list_profiles(&config_path) {
                        Ok(_result) => 0,
                        Err(problem) => { println!("Failed to list profiles <- {}", problem); 1 }
                    });
                }
                Some("create") => {
                    let subsubcommand_matches = subcommand_matches.subcommand_matches("create")
                        .unwrap();
                    let new_profile_name = subsubcommand_matches.value_of("name")
                        .expect("Error reading name of new profile.");
                    exit(match profile::create_profile(&config_path, &new_profile_name) {
                        Ok(_result) => 0,
                        Err(problem) => { println!("Failed to create new profile <- {}", problem); 1 }
                    });
                }
                Some("select") => {
                    let subsubcommand_matches = subcommand_matches.subcommand_matches("select")
                        .unwrap();
                    let new_profile_name = subsubcommand_matches.value_of("name")
                        .expect("Error reading name of selected profile.");
                    exit(match config_file::select_profile(&config_path, &new_profile_name) {
                        Ok(_result) => 0,
                        Err(error) => {println!("Failed to change profile <- {}", error); 1}
                    }
                );
                }
                Some("rm") => {
                    let subsubcommand_matches = subcommand_matches.subcommand_matches("rm")
                        .unwrap();
                    let target_profile_name = subsubcommand_matches.value_of("name")
                        .expect("Error reading name of selected profile.");
                    exit(match profile::remove_profile(&config_path, &target_profile_name) {
                        Ok(_result) => 0,
                        Err(issue) => { println!("Failed to remove profile <- {}", issue); 1 }
                    });
                }
                _ => {
                    println!("Command missing! Try with -h for more info.");
                    exit(1);
                }
            }
        }
        Some("cache") => {
            // Define cache directory because it's used by all commands
            let cache_directory = format!("{}/caches/{}/", &config_path, config_file.current_profile);
            // Check the given subcommand
            let subcommand_matches = matches.subcommand_matches("cache").unwrap();
            // Select a given profile
            match subcommand_matches.subcommand_name() {
                Some("create") => {
                    // Get cache name from command line
                    let subsubcommand_matches  = subcommand_matches.subcommand_matches("create").unwrap();
                    let cache_name = subsubcommand_matches.value_of("name").expect("Error reading name of cache.");
                    // Create cache
                    cache::create_cache(&format!("{}/Data", &current_profile_file.install_path), &cache_directory, &cache_name);
                }
                Some("restore") => {
                    // Get cache name from command line
                    let subsubcommand_matches  = subcommand_matches.subcommand_matches("restore").unwrap();
                    let cache_name = subsubcommand_matches.value_of("name").expect("Error reading name of cache.");
                    cache::restore_cache(&format!("{}/Data", &current_profile_file.install_path), &cache_directory, &cache_name);
                }
                _ => {
                    println!("Command missing! Try with -h for more info.");
                    exit(1);
                }
            }
        }
        Some("install") => {
            let subcommand_matches = matches.subcommand_matches("install")
                .unwrap();
            let force = subcommand_matches.is_present("force");
            // Update repository index if requested
            if subcommand_matches.is_present("update") {
                let repos = &config_file.repository_list;
                for repo in repos {
                    remote::get_index(&repo);
                }
            }
            // Create a queue of mods to install
            let mut mod_queue = Vec::new();
            // Collect requested mods into vector
            for mod_value in subcommand_matches.values_of("name").unwrap() {
                mod_queue.push(mod_value.to_string().clone());
            }
            // Recursively install mods
            loop {
                // Break out of the loop if finished
                if mod_queue.len() == 0 {
                    break;
                }
                // Get current mod from the top of the vector
                let mod_value = mod_queue.pop().unwrap();
                // Install mod
                match mods::install_mod(&config_path, &mod_value, &verbose, &force) {
                    Ok(_) => { println!("Installed {}", &mod_value) },
                    Err(issue) => { println!("Failed to install {} <- {}", &mod_value, &issue); exit(1); }
                }
                // Update file ownership hashmap
                match file_ownership::installation_update(&config_path, &mod_value, &verbose) {
                    Ok(_) => { },
                    Err(issue) => { println!("Failed to update file ownership table <- {}", issue); exit(1) }
                }
                // Push dependencies to stack
                let depends = remote::fetch_mod_depends(&config_path, &config_file.repository_list, &mod_value);
                for item in depends {
                    println!("{} depends on {}", &mod_value, &item);
                    mod_queue.push(item);
                }
                current_profile_file.enabled_mods.push(mod_value.clone());
            }
            // Update profile
            exit(match profile::save_profile_file(&config_path, current_profile_file) {
                Ok(_) => 0,
                Err(issue) => { println!("Failed to save installed mods to profile <- {}", issue); 1 }
            });
        }
        Some("uninstall") => {
            let subcommand_matches = matches.subcommand_matches("uninstall")
                .unwrap();
            for mod_value in subcommand_matches.values_of("name").unwrap() {
                // Remove the mod
                match mods::uninstall_mod(&config_path, &mod_value) {
                    Ok(_) => { println!("Uninstalled {}", &mod_value) },
                    Err(issue) => {
                        println!("Failed to uninstall {} <- {}", &mod_value, issue);
                        exit(1)
                    }
                }
                // Update file ownership dictionary
                match file_ownership::uninstallation_update(&config_path, &mod_value) {
                    Ok(_) => { println!("Updated file ownership table.") },
                    Err(issue) => { println!("Failed to update file ownership table <- {}", issue); exit(1) }
                };
                // Remove mod from profile vector
                
            }
        }
        _ => {
            println!("Command missing! Try with -h for more info.");
            exit(1);
        }
    }
}
