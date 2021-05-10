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
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

use flate2::Compression;
use flate2::write::GzDecoder;
use flate2::write::GzEncoder;
use tar::Archive;
use tar::Builder;

pub(crate) fn create_tarball(_tarball_path: &str, _input_files: &str) -> Result<(), String> {
    // Prepare to build an archive
    let mut buf: Vec<u8> = Vec::new();
    let mut output = Builder::new(&mut buf);
    let input = Path::new(_input_files);

    // Recursively add files, breaking gracefully
    match output.append_dir_all(input.file_name().unwrap(), input) {
        Ok(()) => (),
        Err(issue) => return Err(format!("Failed to build archive from input path recursively <- {}", issue))
    };

    // Finish the tar procedure
    match output.into_inner() {
        Ok(_x) => (),
        Err(issue) => return Err(format!("Failed to finish writing archive to buffer <- {}", issue))
    };

    // Compress the archive/buffer/vector to a file. Buffer implicitly cast/sliced to &[u8]
    let mut gzip = GzEncoder::new(OpenOptions::new().create(true).write(true).open(_tarball_path).unwrap(), Compression::best());
    match gzip.write_all(&mut buf) {
        Ok(()) => (),
        Err(issue) => return Err(format!("Failed to write Gzip <- {}", issue))
    };

    // Close up shop and gfto
    match gzip.try_finish() {
        Ok(()) => (),
        Err(issue) => return Err(format!("Failed to finish gzip procedure <- {}", issue))
    };

    Ok(())
}

pub(crate) fn unpack_tarball(tarball_path: &str, destination_path: &str) -> Result<(), String> {
    // Prepare for gunzip
    let mut filebuf: Vec<u8> = Vec::new();
    let mut gzbuf: Vec<u8> = Vec::new();
    let mut gunzip = GzDecoder::new(&mut gzbuf);

    // Load the file into its buffer
    match File::open(tarball_path).unwrap().read_to_end(&mut filebuf) {
        Ok(_x) => (),
        Err(issue) => return Err(format!("Failed to read archive <- {}", issue))
    };

    // Decompress the file into another buffer
    match gunzip.write_all(&mut filebuf) {
        Ok(()) => (),
        Err(issue) => return Err(format!("Failed to unzip archive <- {}", issue))
    };

    // Finish the gunzip procedure
    match gunzip.try_finish() {
        Ok(()) => (),
        Err(issue) => return Err(format!("Failed to finish gunzip procedure <- {}", issue))
    };

    // Desperation. TODO: see if this sucks
    drop(gunzip);

    // Unroll the archive into a directory
    let mut tarball = Archive::new((&mut gzbuf).as_slice());
    match tarball.unpack(destination_path) {
        Ok(()) => (),
        Err(issue) => return Err(format!("Failed to unpack tar buffer into directory <- {}", issue))
    };

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