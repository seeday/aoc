use std::slice::Iter;
use std::io::{self, Read};
use std::str::FromStr;

fn power_set<'a>(items: &mut Iter<'a,u32>) -> Vec<Vec<u32>> {
    let mut power = Vec::new();
    match items.next() {
        None       => power.push(Vec::new()),
        Some(item) => {
            for mut set in power_set(items).into_iter() {
                power.push(set.clone());
                set.push(item.clone());
                power.push(set);
            }
        }
    }
    power
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut v = Vec::new();
    for line in buf.lines() {
        v.push(u32::from_str(line).unwrap());
    }

    let pset = power_set(&mut v.iter());
    let sumset = pset.iter().fold(0u32, |acc, x| acc + if x.iter().fold(0u32, |acc, x| acc + x) == 150 {1} else {0});
    println!("{:?}", sumset);
    let fillers: Vec<Vec<u32>> = pset.clone().into_iter().filter(|x| x.iter().fold(0u32, |acc, x| acc + x) == 150).collect();
    let min = fillers.iter().min_by_key(|x| x.len()).unwrap().len();
    println!("{}", min);
    let efficient_fillers = fillers.iter().fold(0u32, |acc, x| acc + if x.len() == min {1} else {0});
    println!("{}", efficient_fillers);
}
