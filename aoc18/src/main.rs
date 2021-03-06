use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Matrix {
    size: usize,
    indsize: usize,
    mat: Vec<bool>,
}

impl Matrix {
    fn unfilled(siz: usize) -> Matrix {
        Matrix {
            mat: Vec::with_capacity(siz * siz),
            size: siz,
            indsize: siz - 1,
        }
    }

    fn non(&self, x: usize, y: usize) -> u8 {
        let mut on = 0;

        if x != 0 {
            // left side
            on += self.uget(x - 1, y);
            if y != 0 {
                // check above
                on += self.uget(x - 1, y - 1);
            }
            if y != self.indsize {
                // check below
                on += self.uget(x - 1, y + 1);
            }
        }
        if x != self.indsize {
            // right side
            on += self.uget(x + 1, y);
            if y != 0 {
                // above
                on += self.uget(x + 1, y - 1);
            }
            if y != self.indsize {
                // below
                on += self.uget(x + 1, y + 1);
            }
        }

        // above
        if y != 0 {
            on += self.uget(x, y - 1);
        }
        // below
        if y != self.indsize {
            on += self.uget(x, y + 1);
        }

        on
    }

    fn step(&mut self) {
        let cloned = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                if (x == 0 && y == 0) || (x == self.indsize && y == 0) ||
                   (x == 0 && y == self.indsize) ||
                   (x == self.indsize && y == self.indsize) {
                    continue;
                }
                let non = cloned.non(x, y);
                if cloned.get(x, y) {
                    if !(non == 2 || non == 3) {
                        self.set(x, y, false);
                    }
                } else {
                    if non == 3 {
                        self.set(x, y, true);
                    }
                }
            }
        }
    }

    #[inline]
    fn uget(&self, x: usize, y: usize) -> u8 {
        if self.mat[y * self.size + x] {
            1
        } else {
            0
        }
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> bool {
        self.mat[y * self.size + x]
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.mat[y * self.size + x] = value;
    }

    fn inspect(&self, startx: usize, starty: usize, endx: usize, endy: usize) {
        println!("");
        for y in starty..(endy + 1) {
            for x in startx..(endx + 1) {
                let pos = ((y as usize) * self.size) + (x as usize);
                print!("{}",
                       if self.mat[pos] {
                           "#"
                       } else {
                           "."
                       });//{"Ｘ"} else {"．"});
            }
            println!("");
        }
        println!("");
    }

    fn count(&self) -> usize {
        self.mat.iter().fold(0, |acc, &x| {
            if x {
                acc + 1
            } else {
                acc
            }
        })
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut mat = Matrix::unfilled(100);
    for ch in buf.chars() {
        match ch {
            '#' => mat.mat.push(true),
            '.' => mat.mat.push(false),
            _ => (),
        }
    }

    for _ in 0..100 {
        mat.step();
    }
    println!("{}", mat.count());
}
