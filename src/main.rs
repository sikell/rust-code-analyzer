extern crate walkdir;

use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;
use std::path::Path;

fn main() {
    let analyzed_repo_path = "./";
    let file_extension = ".rs";

    for entry in WalkDir::new(analyzed_repo_path) {
        let entry = entry.unwrap();
        let name = match entry.file_name().to_str() {
            None => "???",
            Some(t) => t
        };
        if name.ends_with(file_extension) {
            println!("{}", entry.path().display());
            open_file(entry.path())
        }
    }
}

fn open_file(file_path: &Path) {
    let file = File::open(file_path).unwrap();
    let lines = BufReader::new(file).lines();
    println!("{} lines in {}", lines.count(), file_path.display());
}
