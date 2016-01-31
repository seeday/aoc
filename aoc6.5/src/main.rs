#[macro_use]
extern crate nom;

use nom::IResult::*;

use std::io::{self, Read};

#[macro_use]
mod parser;
use parser::*;

const SIZE: usize = 1000;

struct Matrix {
    mat: [u16; SIZE*SIZE],
}

impl Matrix {
    fn new() -> Matrix {
        Matrix{mat: [0; SIZE*SIZE]}
    }

    fn modify(&mut self, from: Point, to: Point, op: Op) {
        for y in from.y..(to.y + 1) {
            for x in from.x..(to.x + 1) {
                let pos = ((y as usize) * SIZE) + x as usize;
                match op {
                    parser::Op::Toggle => self.mat[pos] += 2,
                    parser::Op::On => self.mat[pos] += 1,
                    parser::Op::Off => self.mat[pos] = self.mat[pos].saturating_sub(1),
                }
            }
        }
    }

    fn count(&self) -> usize {
        self.mat.iter().fold(0, |acc, &x| (x as usize) + acc)
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

