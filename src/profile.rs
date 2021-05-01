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
pub(crate) struct ProfileFile {
    pub(crate) install_path: String,
    pub(crate) enabled_mods: String,
}
/// `ProfileFile` implements `Default`
impl std::default::Default for ProfileFile {
    fn default() -> Self { Self {
        install_path: "path/to/fallout/install/".into(),
        enabled_mods: "".into(),
    }}
}

pub(crate) fn load_profile_file(profile_path: &str) -> Result<ProfileFile, Box<dyn Error>> {
    // Parse JSON from file
    let profile:ProfileFile = confy::load_path(profile_path)?;
    // Return config
    Ok(profile)
}