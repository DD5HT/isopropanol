use std::path::PathBuf;
use std::fs::remove_file;

fn remove_this_file(files: Vec<PathBuf> ) -> bool {
    for file in files {
        remove_file(file).unwrap();
    }
    true
}