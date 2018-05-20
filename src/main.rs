extern crate walkdir;

use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;
use std::path::Path;
use walkdir::DirEntry;
use walkdir::Error;
use std::collections::LinkedList;

struct Properties<'a> {
    analyzed_repo_path: &'a str,
    file_extension: &'a str,
}

struct FileReport {
    total_lines: usize
}

fn main() {
    let properties = Properties {
        analyzed_repo_path: "./",
        file_extension: ".rs",
    };

    let report: LinkedList<FileReport> = WalkDir::new(properties.analyzed_repo_path)
        .into_iter()
        .map(|e| process_entry(&properties, e))
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect();

    let total_lines: usize = report.into_iter()
        .map(|e| e.total_lines)
        .sum();
    println!("======\nTotal lines: {}", total_lines)
}

fn process_entry(properties: &Properties, entry: Result<DirEntry, Error>) -> Option<FileReport> {
    let entry = entry.unwrap();
    let name = match entry.file_name().to_str() {
        None => "???",
        Some(t) => t
    };
    if name.ends_with(properties.file_extension) {
        return Some(open_file(entry.path()));
    } else {
        None
    }
}

fn open_file(file_path: &Path) -> FileReport {
    let file = File::open(file_path).unwrap();
    let lines = BufReader::new(file).lines();
    let total_lines = lines.count();
    println!("{} lines in {}", total_lines, file_path.display());
    FileReport {
        total_lines
    }
}
