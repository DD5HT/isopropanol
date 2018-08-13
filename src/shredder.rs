use std::fs::{metadata, write, remove_file};
use std::path::PathBuf;

///Overwrites the given file with zeros and delinks it from the file system
pub fn shred(file: &PathBuf) {
    let length = metadata(file).unwrap().len();
    let sample: Vec<u8> = vec![0; length as usize];

    write(file, sample).expect("Can't override the file");
    remove_file(file).expect("Can't remove file!")
}