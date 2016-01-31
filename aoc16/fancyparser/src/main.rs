#[macro_use]
extern crate nom;
use nom::*;
use nom::IResult::*;
use std::io::{self, Read};
use std::str::{self, FromStr, Utf8Error};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialOrd)]
enum Memory {
    Uknown,
    Known(u8),
}

impl Eq for Memory {}
impl PartialEq for Memory {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Memory::Uknown, &Memory::Uknown) => true,
            (&Memory::Known(_), &Memory::Uknown) => true,
            (&Memory::Uknown, &Memory::Known(_)) => true,
            (&Memory::Known(x), &Memory::Known(y)) => x == y,
        }
    }
}

impl Memory {
    fn c_gt(&self, other: &Self) -> bool {
        match (self, other) {
            (&Memory::Uknown, &Memory::Uknown) => true,
            (&Memory::Known(_), &Memory::Uknown) => true,
            (&Memory::Uknown, &Memory::Known(_)) => true,
            (&Memory::Known(x), &Memory::Known(y)) => x < y,
        }
    }

    fn c_lt(&self, other: &Self) -> bool {
        match (self, other) {
            (&Memory::Uknown, &Memory::Uknown) => true,
            (&Memory::Known(_), &Memory::Uknown) => true,
            (&Memory::Uknown, &Memory::Known(_)) => true,
            (&Memory::Known(x), &Memory::Known(y)) => x > y,
        }
    }
}

named!(key<&[u8], &str>, map_res!(alpha, str::from_utf8));

fn to_u8(s: &[u8]) -> Result<u8, Utf8Error> {
    return str::from_utf8(s).map(|x| u8::from_str(x).unwrap());
}
named!(valu8<&[u8], u8>, map_res!(digit, to_u8));

named!(kv<&[u8], (&str, Memory)>, chain!(
        k: key ~
        tag!(":") ~
        space? ~
        v: valu8,
        || (k, Memory::Known(v))
    ));

named!(memories<&[u8], Vec<(&str, Memory)> >, separated_list!(
        chain!(tag!(",") ~ multispace, || ()), 
        kv
    ));

named!(sue<&[u8], Vec<(&str, Memory)> >, chain!(
        tag!("Sue") ~
        space ~ 
        digit ~
        tag!(":") ~
        space ~ 
        m: dbg!(memories) ~
        line_ending ?,
        //|| (str::from_utf8(n).map(|x| u16::from_str(x).unwrap()).unwrap(), m)
        || m
    ));

named!(sues<&[u8], Vec<Vec<(&str, Memory)> > >, many1!(sue));

#[derive(Copy, Clone, Debug)]
struct Sue {
    children: Memory,
    cats: Memory,
    samoyeds: Memory,
    pomeranians: Memory,
    akitas: Memory,
    vizslas: Memory,
    goldfish: Memory,
    trees: Memory,
    cars: Memory,
    perfumes: Memory,
}

impl Eq for Sue {}
impl PartialEq for Sue {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children && self.cats.c_gt(&other.cats) &&
        self.samoyeds == other.samoyeds && self.pomeranians.c_lt(&other.pomeranians) &&
        self.akitas == other.akitas &&
        self.vizslas == other.vizslas && self.goldfish.c_lt(&other.goldfish) &&
        self.trees.c_gt(&other.trees) &&
        self.cars == other.cars && self.perfumes == other.perfumes
    }
}

impl Sue {
    fn new(info: Vec<(&str, Memory)>) -> Self {
        let mut newself = Sue {
            children: Memory::Uknown,
            cats: Memory::Uknown,
            samoyeds: Memory::Uknown,
            pomeranians: Memory::Uknown,
            akitas: Memory::Uknown,
            vizslas: Memory::Uknown,
            goldfish: Memory::Uknown,
            trees: Memory::Uknown,
            cars: Memory::Uknown,
            perfumes: Memory::Uknown,
        };
        for elem in info {
            match elem {
                ("children", x) => newself.children = x,
                ("cats", x) => newself.cats = x,
                ("samoyeds", x) => newself.samoyeds = x,
                ("pomeranians", x) => newself.pomeranians = x,
                ("akitas", x) => newself.akitas = x,
                ("vizslas", x) => newself.vizslas = x,
                ("goldfish", x) => newself.goldfish = x,
                ("trees", x) => newself.trees = x,
                ("cars", x) => newself.cars = x,
                ("perfumes", x) => newself.perfumes = x,
                _ => panic!("Unknown tuple {:?}", elem),
            }
        }
        newself
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let svec = if let Done(remain, parsed) = sues(buf.as_bytes()) {
                   if remain.len() != 0 {
                       panic!("Didn't parse fully! {}", str::from_utf8(remain).unwrap());
                   }
                   Some(parsed.into_iter().map(|x| Sue::new(x)).collect::<Vec<Sue>>())
               } else {
                   None
               }
               .unwrap();

    let mysue = Sue {
        children: Memory::Known(3),
        cats: Memory::Known(7),
        samoyeds: Memory::Known(2),
        pomeranians: Memory::Known(3),
        akitas: Memory::Known(0),
        vizslas: Memory::Known(0),
        goldfish: Memory::Known(5),
        trees: Memory::Known(3),
        cars: Memory::Known(2),
        perfumes: Memory::Known(1),
    };

    for (ind, su) in svec.iter().enumerate() {
        if &mysue == su {
            println!("Sue {}: {:?}", ind + 1, su);
        }
    }
}
