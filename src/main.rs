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
    // Get config path
    let config_path = resolve_home_dir(
        matches.value_of("config")
            .expect("Failed to read config path argument")
    );
    let config_file_path = format!("{}/config", config_path);
    // Create path for config if it doesn't exist
    if !Path::new(&config_path).exists() {
        fs::create_dir_all(&config_path)
            .expect("Failed to create path to configuration directory.");
    }
    // Load configuration file
    let mut config_file = match config_file::load_config_file(&config_file_path) {
        Ok(config_file) => config_file,
        Err(e) => panic!("Failed to load configuration file: {}", e),
    };
    // Get path to current profile
    let profiles_path = format!("{}/profiles", config_path);
    let current_profile_file_path = format!("{}/{}", &profiles_path, &config_file.current_profile);
    let current_profile_file = profile::load_profile_file(&current_profile_file_path)
        .expect("Failed to load current profile!");
    // Execute the given subcommand
    match matches.subcommand_name() {
        Some("profile") => {
            // Check the given subcommand
            let subcommand_matches = matches.subcommand_matches("profile")
                .unwrap();
            // Select a given profile
            match subcommand_matches.subcommand_name() {
                Some("list") | Some("ls") => {
                    let paths = fs::read_dir(profiles_path)
                        .unwrap();
                    println!("Available profiles:");
                    for path in paths {
                        let file_name = path.unwrap().file_name();
                        print!("Profile: {:?}", file_name);
                        // Display an indicator next to the current profile
                        if file_name.to_str().unwrap() == &config_file.current_profile {
                            print!(" [*]\n");
                        } else {
                            print!("\n");
                        }
                    }
                }
                Some("create") => {
                    let subsubcommand_matches = subcommand_matches.subcommand_matches("create")
                        .unwrap();
                    let new_profile_name = subsubcommand_matches.value_of("name")
                        .expect("Error reading name of new profile.");
                    let _new_profile = profile::load_profile_file(&format!("{}/{}", profiles_path, new_profile_name));
                    println!("Created profile {}", new_profile_name);
                }
                Some("select") => {
                    let subsubcommand_matches = subcommand_matches.subcommand_matches("select")
                        .unwrap();
                    let new_profile_name = subsubcommand_matches.value_of("name")
                        .expect("Error reading name of selected profile.");
                    // Test that the new profile exists
                    if !Path::new(&format!("{}/{}", profiles_path, new_profile_name)).exists() {
                        println!("Profile {} does not exist!", new_profile_name);
                        exit(1);
                    } else {
                        config_file.current_profile = new_profile_name.to_string();
                        confy::store_path(config_file_path, config_file)
                            .expect("Error saving configuration file!");
                        println!("Switched to profile {}", new_profile_name);
                    }
                }
                Some("remove") | Some("rm") => {
                    let subsubcommand_matches = subcommand_matches.subcommand_matches("select")
                        .unwrap();
                    let target_profile_name = subsubcommand_matches.value_of("name")
                        .expect("Error reading name of selected profile.");
                    // Ensure the user is not trying to remove their current profile
                    if target_profile_name == config_file.current_profile {
                        // The profile is currently in use
                        println!("Cannot remove current profile!");
                        exit(1);
                    } else {
                        // The profile is not currently in use
                        fs::remove_file(format!("{}/{}", profiles_path, target_profile_name))
                            .expect(&format!("Error removing profile {}", target_profile_name));
                    }

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
            // Update repository index if requested
            if subcommand_matches.is_present("update") {
                let repos = remote::get_repositories(&config_file.repository_list);
                for repo in repos {
                    remote::get_index(&repo);
                }
            }
            let mod_value = subcommand_matches.value_of("name")
                .unwrap();
            // Split modvalue into author and name
            let mod_values = mods::split_mod_value(mod_value);
            let mod_author = &mod_values[0];
            let mod_name = &mod_values[1];
            // Search the mod cache
            if mods::search_mod_cache(&config_path, &mod_value) {
                // Mod exists and can be installed
                if !mods::mod_has_index(&config_path, &mod_value) {
                    // Mod does not have a file index. Generate before install.
                    mods::generate_index(&config_path, &mod_value, matches.is_present("verbose"))
                        .expect("This error shouldn't be possible");
                } else {
                    if matches.is_present("verbose") {
                        println!("Mod {}/{} already has an index file", &mod_author, &mod_name);
                    }
                }
                // Check for file conflicts
                mods::test_file_conflicts(&config_path, &mod_value, &current_profile_file.install_path, matches.is_present("verbose")).expect("Error checking for file conflicts");
                // Install the mod
                mods::install_mod(&config_path, &format!("{}/Data", &current_profile_file.install_path), &mod_value, matches.is_present("verbose"))
            } else {
                // Mod does not exist locally and needs to be fetched
            }
        }
        _ => {
            println!("Command missing! Try with -h for more info.");
            exit(1);
        }
    }
}
