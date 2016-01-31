extern crate permutohedron;
use std::io::{self, Read};
use std::collections::{HashSet, HashMap};
use std::str::FromStr;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut names: HashSet<&str> = HashSet::new();
    let mut dists: HashMap<(&str, &str), u32> = HashMap::new();

    for line in buf.lines() {
        let s: Vec<&str> = line.split(' ').collect();
        names.insert(s[0]);
        names.insert(s[2]);
        dists.insert((s[0], s[2]), u32::from_str(s[4]).unwrap());
        dists.insert((s[2], s[0]), u32::from_str(s[4]).unwrap());
    }

    let mut names: Vec<&str> = names.into_iter().collect();
    let mut min = u32::max_value();
    let mut max = 0;

    permutohedron::heap_recursive(&mut names, |x| {
        let mut dist: u32 = 0;
        for i in 0..x.len() - 1 {
            dist = dist.saturating_add(*dists.get(&(x[i], x[i + 1])).unwrap_or(&u32::max_value()));            
        }
        //println!("{:?}: {}", x, dist);
        if min > dist {
            min = dist;
        }
        if max < dist {
            max = dist;
        }
    });
    println!("min: {} max: {}", min, max);
}
