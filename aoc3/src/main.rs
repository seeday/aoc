use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut set = HashSet::new();
    set.insert((0, 0));
    let mut x = 0;
    let mut y = 0;
    let mut rx = 0;
    let mut ry = 0;
    let mut robot = false;

    for c in buf.chars() {
        let (tx, ty) = match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => (0, 0),
        };
        if robot {
            rx += tx;
            ry += ty;
            set.insert((rx, ry));
        } else {
            x += tx;
            y += ty;
            set.insert((x, y));
        }
        robot = !robot;
    }

    println!("{:?}\n{}", set, set.len());
}
