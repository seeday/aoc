use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut surface = 0;
    let mut ribbon = 0;
    for line in buf.lines() {
        let mut lwh: Vec<u32> = line.split('x').map(|x| x.parse::<u32>().unwrap()).collect();
        lwh.sort();
        assert_eq!(lwh.len(), 3);
        let (l, w, h) = (lwh[0], lwh[1], lwh[2]);

        println!("{:?}\tl*w*h={}\tsface={}", lwh, l * w * h, l + l + w + w);

        surface += 2 * l * w + 2 * w * h + 2 * h * l + l * w;
        ribbon += l * w * h + l + l + w + w;
    }
    println!("{}, {}", surface, ribbon);
}
