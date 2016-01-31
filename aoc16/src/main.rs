extern crate regex;
use regex::Regex;

use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let re = Regex::new(r"children: 3|cats: 7|samoyeds: 2|pomeranians: 3|akitas: 0|vizslas: 0|goldfish: 5|trees: 3|cars: 2|perfumes: 1").unwrap();

    let mut maxmatches = 0;
    let mut thesue: &str = "";
    for line in buf.lines() {
        let matches = re.find_iter(line).count();
        if matches > maxmatches {
            maxmatches = matches;
            thesue = line;
        }
    }

    println!("{}", thesue);
}
