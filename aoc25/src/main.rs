fn main() {
    let mut cur: u64 = 20151125;
    let mut prev: u64 = 20151125;
    let mut count = 0;
    let mut colcount = 1;
    for col in 1..6500 {
        for diag in 1..col+1 {
            if colcount == 3075 {
                println!("col: {} row: {} val: {}", colcount, col - diag + 1, cur);
            }
            prev = cur;
            cur = (prev * 252533) % 33554393;
            count += 1;
            colcount += 1;
            //println!("col: {} row: {} val: {}", colcount, col - diag, cur);
        }
        colcount = 1;
    }
}

