use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut accum = 0;
    let mut position = 0;
    let mut base = None;
    for c in buf.chars() {
        position += 1;
        accum += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if base == None && accum == -1 {
            base = Some(position)
        }
    }
    println!("{}, {:?}", accum, base);
}
