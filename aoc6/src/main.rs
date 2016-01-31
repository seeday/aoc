#[macro_use]
extern crate nom;

use nom::IResult::*;

use std::io::{self, Read};

#[macro_use]
mod parser;
use parser::*;

const SIZE: usize = 1000;

struct Matrix {
    mat: [bool; SIZE*SIZE],
}

impl Matrix {
    fn new() -> Matrix {
        Matrix{mat: [false; SIZE*SIZE]}
    }

    fn modify(&mut self, from: Point, to: Point, op: Op) {
        for y in from.y..(to.y + 1) {
            for x in from.x..(to.x + 1) {
                let pos = ((y as usize) * SIZE) + x as usize;
                self.mat[pos] = match op {
                    parser::Op::Toggle => !self.mat[pos],
                    parser::Op::On => true,
                    parser::Op::Off => false,
                }
            }
        }
    }

    fn inspect(&self, startx: usize, starty: usize, endx: usize, endy: usize) {
        println!("");
        for y in starty..(endy + 1) {
            for x in startx..(endx + 1) {
                let pos = ((y as usize) * SIZE) + (x as usize);
                print!("{}", if self.mat[pos] {"█"} else {"░"});
            }
            println!("");
        }
        println!("");
    }

    fn count(&self) -> usize {
        self.mat.iter().fold(0, |acc, &x| if x {acc + 1} else {acc})
    }
}


fn main() {
    let mut mat = Matrix::new();

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        if let Done(_, op) = parser::full(line.as_bytes()) {
            //println!("{:?}", op);
            mat.modify(op.from, op.to, op.operation);
        }
    }

    println!("{}", mat.count());
}

#[test]
fn mattest() {
    let mut mat = Matrix::new();
    mat.modify(Point{x: 0, y: 0}, Point{x: 10, y: 10}, parser::Op::Toggle);
    assert!(mat.mat[0]);
    assert!(mat.mat[1000]);
    assert!(mat.mat[1001]);
}

#[test]
fn vistest() {
    let mut mat = Matrix::new();
    mat.modify(Point::new(0, 0), Point::new(999, 999), parser::Op::On);
    println!("{}", mat.count());
    mat.modify(Point::new(0, 0), Point::new(999, 999), parser::Op::Off);
    mat.modify(Point::new(0, 0), Point::new(999, 0), parser::Op::On);
    println!("{}", mat.count());
    assert!(false);
}

