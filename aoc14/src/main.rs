#[macro_use]
extern crate scan_fmt;
use std::io::{self, Read};

#[derive(Clone, Debug)]
enum State {
    Flying(u32),
    Resting(u32),
}

#[derive(Clone, Debug)]
struct Reindeer {
    name: String,
    fspeed: u32,
    flength: u32,
    restlen: u32,
    state: State,
    distance: u32,
    time: u32,
    points: u32,
}

impl Reindeer {
    fn new(name: String, speed: u32, dur: u32, restdur: u32) -> Reindeer {
        Reindeer{name: name, fspeed: speed, flength: dur, restlen: restdur, state: State::Flying(dur), distance: 0, time: 0, points: 0}
    }

    fn step(&mut self) -> u32 {
        self.state = match self.state {
            State::Flying(f) => {
                self.distance += self.fspeed;
                self.time += 1;
                if f == 1 {
                    State::Resting(self.restlen)
                } else {
                    State::Flying(f - 1)
                }
            },
            State::Resting(r) => {
                self.time += 1;
                if r == 1 {
                    State::Flying(self.flength)
                } else {
                    State::Resting(r - 1)
                }
            }
        };
        self.distance
    }

    ///Fast forwards to 'to' seconds, restoring simulation accuraccy as 'to' approaches.
    fn ff(&mut self, to: u32) -> &mut Self {
        while self.time != to {
            self.step();
        }
        self
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut plural: Vec<Reindeer> = vec![];
    for line in buf.lines() {
        let (n, s, flen, rlen) = scan_fmt!(line, "{} can fly {} km/s for {} seconds, but then must rest for {} seconds.", String, u32, u32, u32);
        plural.push(Reindeer::new(n.unwrap(), s.unwrap(), flen.unwrap(), rlen.unwrap()));
    }

    let mut max = 0;
    for _ in 0..2503 {
        max = plural.iter_mut().map(|c| c.step()).max().unwrap();
        for r in plural.iter_mut() {
            if r.distance == max {
                r.points += 1;
            } 
        }
    }

    let part1 = plural.iter_mut().max_by_key(|x| x.distance).unwrap().clone();
    let part2 = plural.iter_mut().max_by_key(|x| x.points).unwrap();
    println!("{:?}\n{:?}", part1, part2);
}

#[test]
fn test() {
    let mut comet = Reindeer::new("Comet".to_string(), 14, 10, 127);
    let mut dancer = Reindeer::new("Dancer".to_string(), 16, 11, 162);

    comet.step();
    dancer.step();
    assert_eq!(comet.time, 1);
    assert_eq!(comet.time, dancer.time);
    assert_eq!(comet.distance, 14);
    assert_eq!(dancer.distance, 16);

    comet.ff(1000);
    dancer.ff(1000);

    assert_eq!(comet.distance, 1120);
    assert_eq!(dancer.distance, 1056);
}
