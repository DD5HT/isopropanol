extern crate isopropanol;
extern crate structopt;

use isopropanol::Opt;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
