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

use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;

use super::archives;

pub(crate) fn install_mod(config_path: &str, data_path: &str, mod_author: &str, mod_name: &str, verbose: bool) {
    // Implement this later
}

pub(crate) fn generate_index(config_path: &str, mod_author: &str, mod_name: &str, verbose: bool) -> Result<(), &'static str> {
    // Implement this later
    println!("{}", format!("Generating file index for {}/{}", &mod_author, &mod_name));
    // Create mod path
    let mod_path = format!("{}/mods/cached/{}/{}/mod.tar.gz", &config_path, &mod_author, &mod_name);
    let index_path = format!("{}/mods/indices/{}/{}/index", &config_path, &mod_author, &mod_name);
    // Create indices path if it doesn't exist
    if !Path::new(&index_path).parent().unwrap().exists() {
        fs::create_dir_all(Path::new(&index_path).parent().unwrap()).expect("Unable to create indices folder. Ensure you have permission to do this.");
    }
    if Path::new(&mod_path).exists() {
        let mod_contents = archives::list_contents(&mod_path);
        let mut f = File::create(&index_path).expect("Cannot create index file! Ensure you have permission to do this.");
        for item in mod_contents {
            if verbose {
                println!("{}", item);
            }
            f.write(format!("{}\n", item).as_bytes()).expect("Failed to write index file!");
        }
        Ok(())
    } else {
        Err("Can't index a mod that doesn't exist!")
    }
}

pub(crate) fn mod_has_index(config_path: &str, mod_author: &str, mod_name: &str) -> bool {
    let index_path = format!("{}/mods/indices/{}/{}/index", &config_path, &mod_author, &mod_name);
    return Path::new(&index_path).exists();
}

pub(crate) fn split_mod_value(mod_value: &str) -> Vec<String> {
    // I should be collecting this iterator but I don't know how
    let mut vec = Vec::new();
    for part in mod_value.split("/") {
        vec.push(part.to_string());
    }
    return vec;
}

pub(crate) fn search_mod_cache(config_path: &str, mod_author: &str, mod_name: &str) -> bool {
    // Define mod cache path
    let mod_cache_path: &str = &format!("{}/mods/cached/", &config_path);
    // Ensure the mod cache exists before trying to search it
    if !Path::new(&mod_cache_path).exists() {
        fs::create_dir_all(&mod_cache_path)
            .expect("Failed to create path to mods cache. Ensure you have permissions to do this.");
    }
    // Search the mod cache for a mod
    let mod_path: &str = &format!("{}/{}/{}/mod.tar.gz", &mod_cache_path, &mod_author, &mod_name);
    // Return the value
    return Path::new(&mod_path).exists()
}