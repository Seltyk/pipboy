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

use reqwest;
use std::fs;
use std::path::Path;

use super::config_file;

/// Get the package index of a remote repository
pub(crate) fn get_index(remote: &str) {
    let index_path = format!("Updated index for: https://{}/index.json", remote);
    println!("{}", &index_path);
}

pub(crate) fn fetch_mod(config_path: &str, mod_value: &str) -> Result<(), String> {
    // Get remotes from config file
    let remotes = match config_file::load_config_file(&config_path) {
        Ok(config) => config.repository_list,
        Err(_) => return Err("Failed to load configuration file!".to_string())
    };
    for server in remotes {
        let url = format!("https://{}/mods/{}/mod.tar.gz", &server, &mod_value);
        let res = reqwest::blocking::get(&url).unwrap();
        if res.status().is_success() {
            // The mod was found. Download the mod.
            println!("{} was found at {}", &mod_value, &server);
            let path = format!("{}/mods/cached/{}/", &config_path, &mod_value);
            if !Path::new(&path).exists() {
                match fs::create_dir_all(&path) {
                    Ok(_) => {}
                    Err(_) => return Err(format!("Failed to create path {}", &path))
                };
            }
            return match fs::write(&format!("{}/mod.tar.gz", &path), res.bytes().unwrap()) {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to write mod file to disk".to_string())
            };
        }
    }
    Err("Mod was not found in any repositories".to_string())
}

pub(crate) fn fetch_mod_depends(_config_path: &str, remotes: &Vec<String>, mod_value: &str) -> Vec<String> {
    // TODO: Use game name from profile from remote
    let mut return_vector = Vec::new();
    for server in remotes {
        let url = format!("https://{}/mods/{}/depends.txt", &server, &mod_value);
        let res = reqwest::blocking::get(&url).unwrap();
        if res.status().is_success() {
            let body = res.text().unwrap();
            for dependency in body.lines() {
                return_vector.push(dependency.to_string());
            }
        }
    }
    return return_vector;
}