extern crate regex;
extern crate rand;
use regex::Regex;
use rand::Rng;
use std::collections::HashSet;
use std::io::{self, Read};


fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut fusion: Vec<(&str, &str)> = Vec::new();
    let mut chemical = "";
    for line in buf.lines() {
        let parts: Vec<&str> = line.trim().split(' ').collect();
        if parts.len() == 3 {
            // transformation
            println!("{:<2} => {}", parts[0], parts[2]);
            fusion.push((parts[0].clone(), parts[2].clone()));
        } else if parts.len() == 1 {
            // big chemical
            println!("chem: {}", line);
            chemical = line;
        } else {
            panic!("Unknown number of splits {}: {:?}", parts.len(), parts);
        }
    }

    println!("{}", repset(&chemical.to_string(), &fusion));

    let mut rng = rand::thread_rng();
    loop {
        let (steps, count) = dfs(chemical, &fusion);
        println!("Complete? {}: {} steps", count, steps);
        if count != 1 {
            rng.shuffle(&mut fusion);
        } else {
            break;
        }
    }
}

fn repset(chem: &str, replacevec: &Vec<(&str, &str)>) -> usize {
    let mut acc: HashSet<String> = HashSet::new();
    for &(orig, repl) in replacevec.iter() {
        let re = Regex::new(orig).unwrap();
        for (start, end) in re.find_iter(chem) {
            let prefix = chem[..start].to_string();
            let new = prefix + repl + &chem[end..];
            acc.insert(new);
        }
    }
    acc.len()
}

fn replist(chem: &str, replacevec: &Vec<(&str, &str)>) -> Vec<String> {
    let mut acc = Vec::new();
    for &(orig, repl) in replacevec.iter() {
        let re = Regex::new(repl).unwrap();
        for (start, end) in re.find_iter(chem) {
            let prefix = chem[..start].to_string();
            let new = prefix + orig + &chem[end..];
            acc.push(new);
        }
    }
    acc
}

fn dfs(chem: &str, repl: &Vec<(&str, &str)>) -> (u32, usize) {
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(chem.to_string());

    let mut m = chem.to_string();
    let mut mm = String::new();
    let mut c = 0;
    loop {
        mm.clear();
        for j in replist(&m, &repl) {
            if visited.contains(&j) {
                continue;
            }
            mm = j.clone();
            visited.insert(j);
            break;
        }
        if mm.len() == 0 {
            //println!("Failed to {}", m);
            return (c, m.len());
        } else if mm.len() == 1 {
            return (c + 1, 1);
        }
        m = mm.clone();
        c += 1;
        //println!("Step {}: {}", c, m);
    }
}
