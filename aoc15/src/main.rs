#[macro_use]
extern crate clap;
extern crate rand;
extern crate time;
#[macro_use]
extern crate scan_fmt;

use std::io::{self, Read};

mod algorithm;
mod individual;
mod problem;
use problem::*;

#[derive(Debug, Copy, Clone)]
pub struct Parameters {
    dimension: usize,
    population: usize,
    iterations: usize,
    selection: usize,
    elitism: usize,
    mutation: f64,
    crossover: f64,
    verbosity: usize,
}

fn main() {

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut ing: Vec<Ingredient> = vec![];
    for line in buf.lines() {
        let (_, cap, dur, fla, tex, cal) = scan_fmt!(line,
                                                        "{}: capacity {}, durability {}, flavor \
                                                         {}, texture {}, calories {}",
                                                        String,
                                                        i8,
                                                        i8,
                                                        i8,
                                                        i8,
                                                        i8);
        ing.push(Ingredient {
            capacity: cap.unwrap(),
            durability: dur.unwrap(),
            flavor: fla.unwrap(),
            texture: tex.unwrap(),
            calories: cal.unwrap(),
        });
    }

    println!("{:?}", ing);
    let problem = Problem {
        dom: (0, 100),
        ingredients: ing,
    };

    let parameters = Parameters {
        dimension: problem.ingredients.len(),
        population: 1024,
        iterations: 100000,
        selection: 4,
        elitism: 2,
        mutation: 0.8_f64,
        crossover: 0.8_f64,
        verbosity: 1,
    };

    let results = algorithm::search(&problem, parameters);
    println!("{:?} converged to {} after {} generations in {} seconds.",
             results.problem,
             results.individual.fitness,
             results.iterations,
             results.duration);
    println!("{:?}", results.individual.solution);
}
