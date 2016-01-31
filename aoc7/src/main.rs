#![feature(convert)]
#[macro_use]
extern crate nom;

use std::io::{self, Read};

mod plexer;
mod eval;
use eval::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut e = Eval::new();
    e.load(buf.as_bytes());
    e.solve();
    // e.dumpenv();
    println!("a: {}", e.eval("a"));

    // part 2
    let temp = e.eval("a");
    e.env.clear();
    e.env.insert("b", temp);
    e.solve();
    e.dumpenv();
    println!("a(2): {}", e.eval("a"));
}
