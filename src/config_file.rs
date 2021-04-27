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

#[derive(Serialize, Deserialize)]
pub(crate) struct ConfigFile {
    pub(crate) current_profile: String,
}
/// `ConfigFile` implements `Default`
impl std::default::Default for ConfigFile {
    fn default() -> Self { Self {
        current_profile: "Fallout New Vegas".into(),
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
    let config:ConfigFile = confy::load_path(config_path)?;
    // Return config
    Ok(config)
}