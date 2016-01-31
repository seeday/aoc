#![feature(convert)]
#![feature(slice_patterns)]
extern crate permutohedron;

use std::io::{self, Read};
use std::str::FromStr;
use std::collections::HashSet;
use std::collections::HashMap;

fn parse(lines: &String) -> (Vec<String>, HashMap<String, HashMap<String, i64>>) {
    let mut prefs2: HashMap<String, HashMap<String, i64>> = HashMap::new();
    let mut people = HashSet::new();
    for line in lines.lines() {
        let r: Vec<&str> = line.trim().trim_matches('.').split(" ").collect();
        people.insert(r[0].to_string());
        if let [who, _, gl, units, _, _, _, _, _, _, nextto] = r.as_slice() {
            let swho = who.to_string();
            let next = nextto.to_string();
            let score = i64::from_str(units).unwrap() * {if gl == "gain" {1} else if gl == "lose" {-1} else {panic!("not gain or lose")}};
            if prefs2.contains_key(&swho) {
                let nest = prefs2.get_mut(&swho).unwrap();
                (*nest).insert(next, score);
            } else {
                let mut x = HashMap::new();
                x.insert(next, score);
                prefs2.insert(swho, x);
            }
        }
    }
    (people.into_iter().collect::<Vec<String>>(), prefs2)
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let (mut people, prefs) = parse(&buf);

    println!("{}", prefs.len());
    println!("{:?}", prefs);
    let mut highscore = 0;
    let mut arrange: Vec<String> = vec![];
    let len = people.len();

    permutohedron::heap_recursive(&mut people, |perm| {
        //score seating
        //sitting in a circle, so first and last are also next to eachother
        let mut score: i64 = 0;
        for (ind, p) in perm.iter().enumerate() {
            let map = prefs.get(p).unwrap();
            if ind == 0 {
                score += *map.get(&perm[1]).unwrap();
                score += *map.get(&perm[len - 1]).unwrap();
            } else if ind == len - 1 {
                score += *map.get(&perm[0]).unwrap();
                score += *map.get(&perm[ind - 1]).unwrap();
            } else {
                score += *map.get(&perm[ind + 1]).expect(&format!("didn't find relation for {} and {}", p, perm[ind + 1]));
                score += *map.get(&perm[ind - 1]).expect(&format!("didn't find relation for {} and {}", p, perm[ind + 1]));
            }
        }
        if score > highscore {
            highscore = score;
            arrange = perm.to_vec();
            println!("highscore: {}", highscore);
        }
    });
    println!("{:?}: {}", arrange, highscore);
}
