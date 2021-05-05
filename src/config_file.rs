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

use std::error::Error;
use confy;
use serde::{Serialize, Deserialize};

use super::profile;

#[derive(Serialize, Deserialize)]
pub(crate) struct ConfigFile {
    pub(crate) current_profile: String,
    pub(crate) repository_list: String,
}
/// `ConfigFile` implements `Default`
impl std::default::Default for ConfigFile {
    fn default() -> Self { Self {
        current_profile: "Fallout New Vegas".into(),
        repository_list: "pipboy.aayla.dev".into(),
    }}
}
/// Returns a ConfigFile from a given path
/// # Arguments
/// 1. config_path - The path to the file to load the key from
/// # Examples
/// ```rs
/// let config = load_auth_file("config.conf").unwrap();
/// ```
pub(crate) fn load_config_file(config_path: &str) -> Result<ConfigFile, Box<dyn Error>> {
    // Parse JSON from file
    let config:ConfigFile = confy::load_path(&format!("{}/profile", config_path))?;
    // Return config
    Ok(config)
}

pub(crate) fn save_config_file(config_path: &str, new_config: ConfigFile) -> Result<(), &'static str> {
    return match confy::store_path(&format!("{}/config", &config_path), &new_config) {
        Ok(_result) => Ok(()),
        Err(_error) => Err("Failed to save config file.")
    }
}

pub(crate) fn select_profile(config_path: &str, new_profile: &str) -> Result<(), &'static str> {
    if profile::profile_exists(&config_path, &new_profile) {
        // Load configuration file
        let mut config_file = match load_config_file(&config_path) {
            Ok(config) => config,
            Err(_error) => return Err("Failed to load config file")
        };
        config_file.current_profile = new_profile.to_string();
        return  match save_config_file(config_path, config_file) {
            Ok(_result) => Ok(()),
            Err(_error) => Err(_error)
        }
    } else {
        return Err("Profile does not exist!");
    }
}

pub(crate) fn current_profile(config_path: &str) -> Result<String, &'static str> {
    return match load_config_file(&format!("{}/config", &config_path)) {
        Ok(config) => Ok(config.current_profile),
        Err(_problem) => return Err("Failed to load configuration file!")
    };
}