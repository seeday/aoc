#![feature(iter_arith)]

fn main() {
    let weights: Vec<u32> = vec![1, 2, 3, 5, 7, 13, 17, 19, 23, 29, 31, 37, 41, 43, 53, 59,
    61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];

    let sum: u32 = weights.iter().sum();
    let sz = sum / 4;

    for i in 0..weights.len() {
        let mut v = Vec::new();
        for c in combinations(&weights, i) {
            if c.iter().sum::<u32>() == sz {
                //println!("Got sum for values {:?}", c);
                //println!("Pushed {}", c.iter().fold(1u64, |acc, x| acc * (*x as u64)));
                v.push(c.iter().fold(1u64, |acc, x| acc * (*x as u64)));
            }
        }
        println!("Min QE for i={}: {:?}", i, v.iter().min());
    }
}

fn combinations<T: Copy>(arr: &[T], n: usize) -> Vec<Vec<T>> {
    fn intern<T: Copy>(arr: &[T], n: usize, incl_arr: &mut[bool], index: usize) -> Vec<Vec<T>> {
        if arr.len() < n + index {
            return Vec::new();
        }
        if n == 0 {
            let it = arr.iter().zip(incl_arr.iter()).filter_map(|(val, incl)| {
                if *incl {
                    Some(*val)
                } else {
                    None
                }
            }).collect();
            let mut r = Vec::new();
            r.push(it);
            return r;
        } else {
            let mut r = Vec::new();
            incl_arr[index] = true;
            let mut one = intern(arr, n - 1, incl_arr, index + 1);
            r.append(&mut one);
            incl_arr[index] = false;
            let mut two = intern(arr, n, incl_arr, index + 1);
            r.append(&mut two);
            return r;
        }
    }
    let mut incl_arr = Vec::with_capacity(arr.len());
    for _ in 0..arr.len() {
        incl_arr.push(false);
    }
    intern(arr, n, &mut incl_arr, 0)
}
