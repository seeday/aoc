#[macro_use]
extern crate nom;

use std::io::{self, Read};

mod plexer;
mod eval;
use eval::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut e = Eval::load(buf);
    e.run();
    println!("{:?}", e);
}
