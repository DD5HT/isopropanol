extern crate isopropanol;

use isopropanol::shredder::shred;
use std::path::PathBuf;
#[test]
fn it_shreds_file() {
    let mut path = PathBuf::new();
    path.push("./sample.txt");
    shred(&path);
}
