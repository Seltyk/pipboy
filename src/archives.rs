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

use std::fs::File;
use std::error::Error;
use tar::Archive;

pub(crate) fn create_tarball(_tarball_path: &str, _input_files: &str) -> Result<(), std::io::Error> {
    // TODO: Reimplement this function
    Ok(())
}

pub(crate) fn unpack_tarball(tarball_path: &str, destination_path: &str) -> Result<(), Box<dyn Error>> {
    // Ensure the tarball exists
    let mut tarball = Archive::new(File::open(&tarball_path).unwrap());
    tarball.unpack(&destination_path)?;
    Ok(())
}

pub(crate) fn list_contents(tarball_path: &str) -> Vec<String> {
    let mut return_vector = Vec::new();
    let mut ar = Archive::new(File::open(&tarball_path).unwrap());
    let ar_entries = ar.entries().unwrap();
    for item in ar_entries {
        let file = item.unwrap();
        let file_path = file.header().path().unwrap();
        return_vector.push(file_path.to_str().unwrap().to_string())
    }
    return return_vector;
}