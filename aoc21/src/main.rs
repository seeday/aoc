#[macro_use]
extern crate itertools;
use std::cmp;
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Fighter {
    health: u8,
    damage: u8,
    armor: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Equip {
    dam: u8,
    arm: u8,
    cost: u16,
}

impl Add for Equip {
    type Output = Equip;

    fn add(self, rhs: Self) -> Self {
        Equip{dam: self.dam + rhs.dam, arm: self.arm + rhs.arm, cost: self.cost + rhs.cost}
    }
}

fn main() {
    let weps = vec![Equip{cost: 8, dam: 4, arm: 0}, Equip{cost: 10, dam: 5, arm: 0}, Equip{cost: 25, dam: 6, arm: 0}, Equip{cost: 40, dam: 7, arm: 0}, Equip{cost: 74, dam: 8, arm: 0}];
    let arms = vec![Equip{cost: 0, dam: 0, arm: 0}, Equip{cost: 13, dam: 0, arm: 1}, Equip{cost: 31, dam: 0, arm: 2}, Equip{cost: 53, dam: 0, arm: 3}, Equip{cost: 75, dam: 0, arm: 4}, Equip{cost: 102, dam: 0, arm: 5}];
    let rings = vec![Equip{cost: 0, dam: 0, arm: 0}, Equip{cost: 25, dam: 1, arm: 0}, Equip{cost: 50, dam: 2, arm: 0}, Equip{cost: 100, dam: 3, arm: 0}, Equip{cost: 20, dam: 0, arm: 1}, Equip{cost: 40, dam: 0, arm: 2}, Equip{cost: 80, dam: 0, arm: 3}];
    println!("{}", min_cost_to_win(min_stats_for_boss(), weps.clone(), arms.clone(), rings.clone()));
    println!("{}", max_cost_to_lose(stats_that_lose(), weps, arms, rings));
}

fn min_cost_to_win(combos: Vec<(u8, u8)>, weps: Vec<Equip>, arms: Vec<Equip>, rings: Vec<Equip>) -> u32 {
    //use itertools cartesian product to generate all possibilities, then saving those values so we
    //don't have to compute them again, find the minimum price for ALL of the different winning
    //armor combinations
    let mut allequips: Vec<Equip> = Vec::new();
    for (wep, arm, ring1, ring2) in iproduct!(weps.iter(), arms.iter(), rings.iter(), rings.iter()) {
        if ring1 != ring2 || (*ring1 == Equip{cost: 0, dam: 0, arm: 0}) {
            allequips.push(*wep + *arm + *ring1 + *ring2);
        }
    }
    
    let mut cheapest = u32::max_value();
    for (damage, armor) in combos {
        let cheap = allequips.iter().min_by_key(|e| if e.dam == damage && e.arm == armor {e.cost as u32} else {u32::max_value()});
        println!("Lowest cost for ({}, {}) is {:?}", damage, armor, cheap);
        if let Some(c) = cheap {
            if (c.cost as u32) < cheapest {
                cheapest = c.cost as u32;
            }
        }
    }
    cheapest
}

fn min_stats_for_boss() -> Vec<(u8, u8)> {
    let boss = Fighter{health: 109, damage: 8, armor: 2};
    let health = 100;
    
    let mut vec = Vec::new();
    for damage in 4..12 {
        for armor in 0..9 {
            let player = Fighter{health: health, damage: damage, armor: armor};
            if simfight(player, boss) < simfight(boss, player) {
                vec.push((damage, armor));
                println!("damage: {} armor: {} beats boss", damage, armor);
            }
        }
    }

    vec
}

fn max_cost_to_lose(combos: Vec<(u8, u8)>, weps: Vec<Equip>, arms: Vec<Equip>, rings: Vec<Equip>) -> u32 {
    //use itertools cartesian product to generate all possibilities, then saving those values so we
    //don't have to compute them again, find the minimum price for ALL of the different winning
    //armor combinations
    let mut allequips: Vec<Equip> = Vec::new();
    for (wep, arm, ring1, ring2) in iproduct!(weps.iter(), arms.iter(), rings.iter(), rings.iter()) {
        if ring1 != ring2 || (*ring1 == Equip{cost: 0, dam: 0, arm: 0}) {
            allequips.push(*wep + *arm + *ring1 + *ring2);
        }
    }
    
    let mut expensivest = 0;
    for (damage, armor) in combos {
        let exp = allequips.iter().max_by_key(|e| if e.dam == damage && e.arm == armor {e.cost as u32} else {0});
        println!("Highest cost for ({}, {}) is {:?}", damage, armor, exp);
        if let Some(c) = exp {
            if (c.cost as u32) > expensivest {
                expensivest = c.cost as u32;
            }
        }
    }
    expensivest
}

fn stats_that_lose() -> Vec<(u8, u8)> {
    let boss = Fighter{health: 109, damage: 8, armor: 2};
    let health = 100;
    
    let mut vec = Vec::new();
    for damage in 4..15 {
        for armor in 0..11 {
            let player = Fighter{health: health, damage: damage, armor: armor};
            if simfight(player, boss) > simfight(boss, player) {
                vec.push((damage, armor));
                println!("damage: {} armor: {} loses to boss", damage, armor);
            }
        }
    }

    vec
}

fn simfight(attacker: Fighter, defender: Fighter) -> u32 {
    (defender.health as f32 / cmp::max(attacker.damage.saturating_sub(defender.armor), 1) as f32).ceil() as u32
}

