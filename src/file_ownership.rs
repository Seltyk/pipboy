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
    // Generate mod index if it doesn't exist
    if !mods::mod_has_index(&config_path, &mod_value) {
        match mods::generate_index(&config_path, &mod_value, &verbose) {
            Ok(_) => println!("Generated index for {}", &mod_value),
            Err(issue) => return Err(format!("Failed to generate mod index for {} <- {}", &mod_value, issue))
        }
    }
    // Load mod index
    let mod_index = match mods::load_index(&config_path, &mod_value) {
        Ok(index) => index,
        Err(issue) => return Err(format!("Failed to get mod index <- {}", issue))
    };
    // If we've gotten this far, the user has either ignored checking or the ownership
    // table is out of sync, so it's safe to just nuke and overwrite duplicate entries.
    for file in mod_index.lines() {
        // Remove the value if the file was already in use by another mod
        if ownership_map.contains_key(&*file) {
            ownership_map.remove(&*file);
        }
        // Define the file's owner
        ownership_map.insert(
            file.to_string(),
            mod_value.to_string(),
        );
    }
    // Serialize the dictionary
    return match save_ownership_hashmap(&config_path, ownership_map) {
        Ok(_) => Ok(()),
        Err(issue) => return Err(format!("Failed to save ownership dictionary <- {}", issue))
    };
}

pub(crate) fn uninstallation_update(config_path: &str, mod_value: &str) -> Result<(), String> {
    // Get existing HashMap
    let mut ownership_map = match load_ownership_hashmap(&config_path) {
        Ok(map) => map,
        Err(issue) => return Err(format!("Failed to load file ownership table <- {}", issue))
    };
    // Get index for mod
    let mod_index = match mods::load_index(&config_path, &mod_value) {
        Ok(index) => index,
        Err(issue) => return Err(format!("Failed to get mod index <- {}", issue))
    };
    for file in mod_index.lines() {
        if ownership_map.contains_key(&*file) {
            if ownership_map.get(&*file).unwrap() == &mod_value {
                ownership_map.remove(&*file);
            }
        }
    }
    return match save_ownership_hashmap(&config_path, ownership_map) {
        Ok(_) => Ok(()),
        Err(issue) => Err(format!("Failed to save ownership table <- {}", issue))
    }
}