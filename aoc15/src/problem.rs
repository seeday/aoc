/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

use rand::distributions::Range;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Ingredient {
    // no name field because of lifetime shittiness
    pub capacity: i8,
    pub durability: i8,
    pub flavor: i8,
    pub texture: i8,
    pub calories: i8,
}

pub trait Problematic {
    type G: Clone;
    type D: Clone + Copy;
    type F: Clone + Ord + PartialOrd + Eq + PartialEq;

    fn fitness(&self, genes: &Self::G) -> Self::F;
    fn domain(&self) -> (Self::D, Self::D);
    fn domain_dist(&self) -> Range<Self::D>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Problem {
    pub dom: (u32, u32),
    pub ingredients: Vec<Ingredient>,
}

impl Problematic for Problem {
    type D = u8;
    type G = Vec<u8>;
    type F = i64;

    fn fitness(&self, x: &Vec<u8>) -> i64 {
        assert_eq!(self.ingredients.len(), x.len()); //number of genes same as number of ingredients

        if x.iter().fold(0u64, |acc, &y| acc + (y as u64)) > 100 {
            // println!("Threw out {:?}", x);
            return 0;
        }

        let mut cap = 0;
        let mut dur = 0;
        let mut fla = 0;
        let mut tex = 0;
        let mut cal = 0;
        for (ing, amt) in self.ingredients.iter().zip(x.iter()) {
            cap += ing.capacity as i64 * *amt as i64;
            dur += ing.durability as i64 * *amt as i64;
            fla += ing.flavor as i64 * *amt as i64;
            tex += ing.texture as i64 * *amt as i64;
            cal += ing.calories as i64 * *amt as i64;
        }
        if cap < 0 || dur < 0 || fla < 0 || tex < 0 || cal < 0 || cal != 500 {
            0
        } else {
            cap * dur * fla * tex
        }
    }

    /// Domain for the given problem.
    fn domain(&self) -> (u8, u8) {
        (0, 100)
    }

    /// Random distribution for problem's domain
    fn domain_dist(&self) -> Range<u8> {
        let (a, b) = self.domain();
        Range::new(a, b)
    }
}
