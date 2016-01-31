extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut mincost = u32::max_value();
    let mut moves = Vec::new();

    loop {
        let mut bhp = 51;
        let bdam = 9;

        let mut hp = 50;
        let mut armor = 0;
        let mut mana = 500;
        let mut mcost = 0;
        let mut shield = 0;
        let mut poison = 0;
        let mut recharge = 0;
        moves.clear();

        loop {
            //hp -= 1; //hard mode (part 2)
            //if hp <= 0 {
            //    break;
            //}
            if shield > 0 {
                armor = 7;
                shield -= 1;
            } else {
                armor = 0;
            }
            if poison > 0 {
                bhp -= 3;
                poison -= 1;
                //check if boss died to poison
                if bhp <= 0 {
                    if mcost < mincost {
                        mincost = mcost;
                        println!("Mincost: {} moves: {:?}", mincost, moves);
                    }
                }
            } 
            if recharge > 0 {
                mana += 101;
                recharge -= 1;
            }

            loop {
                //inner loop to make sure we actually use a move
                match rng.gen_range(0usize, 5usize) {
                    0 => {
                        if mana >= 53 {
                            bhp -= 4;
                            mana -= 53;
                            mcost += 53;
                            moves.push(0);
                            break;
                        } else {
                            //this one is lowest cost, so just fail
                            break;
                        }
                    }, //magic missile
                    1 => {
                        if mana >= 73 {
                            bhp -= 2;
                            hp += 2;
                            mana -= 73;
                            mcost += 73;
                            moves.push(1);
                            break;
                        } else {
                            continue;
                        }
                    }, //drain
                    2 => {
                        if shield == 0 && mana >= 113 {
                            shield = 6;
                            mana -= 113;
                            mcost += 113;
                            moves.push(2);
                            break;
                        } else {
                            continue;
                        }
                    }, //shield
                    3 => {
                        if poison == 0 && mana >= 173 {
                            poison = 6;
                            mana -= 173;
                            mcost += 173;
                            moves.push(3);
                            break;
                        } else {
                            continue;
                        }
                    }, //poison
                    4 => {
                        if recharge == 0 && mana >= 229 {
                            recharge = 5;
                            mana -= 229;
                            mcost += 229;
                            moves.push(4);
                            break;
                        } else {
                            continue;
                        }
                    }, //recharge
                    _ => panic!("gen_range doesn't work how it should"),
                }
            }

            //boss turn
            if shield > 0 {
                armor = 7;
                shield -= 1;
            } else {
                armor = 0;
            }
            if poison > 0 {
                bhp -= 3;
                poison -= 1;
                //check if boss died to poison
                if bhp <= 0 {
                    if mcost < mincost {
                        mincost = mcost;
                        println!("Mincost: {} moves: {:?}", mincost, moves);
                    }
                }
            } 
            if recharge > 0 {
                mana += 101;
                recharge -= 1;
            }
            if bhp <= 0 {
                if mcost < mincost {
                    mincost = mcost;
                    println!("Mincost: {} moves: {:?}", mincost, moves);
                }
            }
            hp -= bdam - armor;
            if mana < 53 {
                //println!("Player ran out of mana");
                break;
            }
            if hp <= 0 {
                //println!("Player ran out of health");
                break;
            }
        }
    }
}
