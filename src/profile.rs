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
use std::fs;
use std::path::Path;

use super::config_file;

#[derive(Serialize, Deserialize)]
pub(crate) struct ProfileFile {
    pub(crate) install_path: String,
    pub(crate) enabled_mods: String,
    pub(crate) game: String,
}
/// `ProfileFile` implements `Default`
impl std::default::Default for ProfileFile {
    fn default() -> Self { Self {
        install_path: "path/to/fallout/install/".into(),
        enabled_mods: "".into(),
        game: "Fallout: New Vegas".into(),
    }}
}

pub(crate) fn load_profile_file(profile_path: &str) -> Result<ProfileFile, Box<dyn Error>> {
    // Parse JSON from file
    let profile:ProfileFile = confy::load_path(profile_path)?;
    // Return config
    Ok(profile)
}

pub(crate) fn list_profiles(config_path: &str) {
    let config_file = config_file::load_config_file(&format!("{}/config", &config_path))
        .unwrap();
    let paths = fs::read_dir(format!("{}/profiles/", &config_path))
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

fn profile_exists(config_path: &str, profile_name: &str) -> bool {
    false
}

pub(crate) fn create_profile(config_path: &str, profile_name: &str) {
    // Ensure the profile doesn't already exist before attempting to create it
    if !profile_exists(&config_path, &profile_name) {
        // Define the path to the new profile
        let profile_path = format!("{}/profiles/{}/profile", &config_path, &profile_name);
        // Create prerequisite path
        if !Path::new(&profile_path).parent().unwrap().exists() {
            fs::create_dir_all(&profile_path).expect("Failed to create directories for profile");
        }
        load_profile_file(&profile_path).expect("Failed to create new profile!");
    } else {
        println!("Profile {} already exists!", &profile_name);
    }
}