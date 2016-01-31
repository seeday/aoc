extern crate regex;
use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let re = Regex::new(r"\\x[0-9a-fA-F]{2}|\\[^a-zA-Z0-9]").unwrap();

    let mut codech = 0;
    let mut stringch = 0;
    let mut escch = 0;
    for line in buf.lines() {
        let cl = line.len();
        let sl = re.replace_all(line, "|").trim_matches('"').len();
        let el = format!("{:?}", line).len();
        println!("{:<20}\tcl: {:>2} sl: {:>2}", line, cl, sl);
        codech += cl;
        stringch += sl;
        escch += el;
    }
    println!("code: {}\nstring: {}\nescaped: {}\ncode - string: {}\nescaped - code: {}", codech, stringch, escch, codech - stringch, escch - codech);
}
