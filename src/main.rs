extern crate walkdir;

use walkdir::WalkDir;

fn main() {
    let analyzed_repo_path = "./";
    
    for entry in WalkDir::new(analyzed_repo_path) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
}
