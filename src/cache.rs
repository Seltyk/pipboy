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

use super::archives;

use std::process::exit;
use std::path::Path;
use std::fs;

pub(crate) fn create_cache(data_path: &str, cache_directory: &str, cache_name: &str) {
    // Test that data path is real
    if !Path::new(&data_path).is_dir() {
        println!("{} is not a valid path!", &data_path);
        exit(1);
    }
    // Create cache folder if necessary
    if !Path::new(&cache_directory).exists() {
        fs::create_dir_all(&cache_directory).expect(&format!("Error creating {}", &cache_directory));
    }
    // Ensure the file doesn't already exist
    let cache_path = &format!("{}/{}.tar.gz", cache_directory, &cache_name);
    if Path::new(&cache_path).exists() {
        println!("Cache {} already exists!", &cache_name);
        exit(1);
    }
    // Tarball the directory contents
    archives::create_tarball(&cache_path, &data_path).expect("Error caching Data directory. Do not install mods.");
}

pub(crate) fn restore_cache(data_path: &str, cache_directory: &str, cache_name: &str) {
    let cache_path = &format!("{}/{}.tar.gz", &cache_directory, &cache_name);
    // Test that a cache to restore from exists
    if !Path::new(cache_path).exists() {
        println!("Cache {} does not exist!", &cache_name);
        exit(1);
    }
    // Test that the data path exists before attempting to remove
    if Path::new(&data_path).is_dir() {
        println!("Removing existing data directory.");
        fs::remove_dir_all(&data_path).expect("Error deleting Data/ folder. Make sure you have permissions to do this.");
    }
    // Create new data folder
    fs::create_dir(&data_path).expect("Error creating new Data/ folder. Make sure you have permissions to do this.");
    // Unpack tarball
    println!("Restoring cache.");
    archives::unpack_tarball(&cache_path, &data_path).expect("Error restoring cache");
}