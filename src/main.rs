extern crate walkdir;

use walkdir::WalkDir;

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
        }
    }
}
