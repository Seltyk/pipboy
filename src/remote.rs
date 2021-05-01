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

/// Get the package index of a remote repository
pub(crate) fn get_index(remote: &str) {
    let index_path = format!("Updated index for: https://{}/index.json", remote);
    println!("{}", &index_path);
}

pub(crate) fn get_repositories(csv: &str) -> Vec<String> {
    // I should use a collection here but I'm not sure how
    let mut vec = Vec::new();
    for repo in csv.split(",") {
        vec.push(repo.to_string());
    }
    return vec;
}