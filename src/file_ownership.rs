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

use super::config_file;
use super::mods;

use serde_json;
use std::collections::HashMap;
use std::path::Path;
use std::fs;

fn save_ownership_hashmap(config_path: &str, new_map: HashMap<String, String>) -> Result<(), String> {
    let config_file = match config_file::load_config_file(&config_path) {
        Ok(config) => config,
        Err(_) => return Err("Failed to load configuration file!".to_string())
    };
    let ownership_path = format!("{}/profiles/{}/file_ownership.json", &config_path, &config_file.current_profile);
    // Serialize the dictionary
    let j = serde_json::to_string(&new_map).unwrap();
    return match fs::write(&ownership_path, &j) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to write ownership dictionary to disk.".to_string())
    }
}

fn load_ownership_hashmap(config_path: &str) -> Result<HashMap<String, String>, String> {
    let config_file = match config_file::load_config_file(&config_path) {
        Ok(config) => config,
        Err(_) => return Err("Failed to load configuration file!".to_string())
    };
    let ownership_path = format!("{}/profiles/{}/file_ownership.json", &config_path, &config_file.current_profile);
    // Define the map that will be returned
    let mut return_map = HashMap::new();
    // Populate the return map
    if Path::new(&ownership_path).exists() {
        let j: String = fs::read_to_string(&ownership_path).unwrap().parse().unwrap();
        let dict_load: HashMap<String, String> = serde_json::from_str(&j).unwrap();
        for item in dict_load {
            return_map.insert(item.0, item.1);
        }
    }
    Ok(return_map)
}

pub(crate) fn installation_update(config_path: &str, mod_value: &str, verbose: &bool) -> Result<(), String> {
    // Get existing HashMap
    let mut ownership_map = match load_ownership_hashmap(&config_path) {
        Ok(map) => map,
        Err(issue) => return Err(format!("Failed to load file ownership table <- {}", issue))
    };
    // Define path to mod index
    let index_path = format!("{}/mods/indices/{}/index", &config_path, &mod_value);
    // Test that mod index exists
    if !Path::new(&index_path).exists() {
        // Generate index if it doesn't exist
        println!("Index for {} does not exist. Generating.", &mod_value);
        match mods::generate_index(&config_path, &mod_value, &verbose) {
            Ok(_) => println!("Generated index for {}", &mod_value),
            Err(issue) => return Err(format!("Failed to generate mod index for {} <- {}", &mod_value, issue))
        }
    }
    // Load mod index
    let mod_index: String = fs::read_to_string(&index_path)
        .unwrap().parse().unwrap();
    // Populate HashMap with updates
    for file in mod_index.lines() {
        let file_string = file.to_string();
        // Remove the value if the file was already in use by another mod
        if ownership_map.contains_key(&file_string) {
            ownership_map.remove(&file_string);
        }
        // Define the file's owner
        ownership_map.insert(
            file_string,
            mod_value.to_string(),
        );
    }
    // Serialize the dictionary
    return match save_ownership_hashmap(&config_path, ownership_map) {
        Ok(_) => Ok(()),
        Err(issue) => return Err(format!("Failed to save ownership dictionary <- {}", issue))
    };
}