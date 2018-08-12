use std::fs::{metadata, write};
use std::path::PathBuf;
use std::io::prelude::*;

pub fn shred(file: &PathBuf) {
    let attr = metadata(file).unwrap().len();
    println!("{:?} bytes", attr);

    let b_slice: &[u8] = &[0,0,0,0];

    write(file, b_slice).unwrap();
}