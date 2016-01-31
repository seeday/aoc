#![feature(step_by)]

fn main() {
    presentsforhouse();
    //lazyelves();
}

fn presentsforhouse() {
    let mut presents = Vec::with_capacity(3_400_000);
    for _ in 0..presents.capacity() {
        presents.push(10);
    }

    for elf in 2..presents.len() {
        for house in (elf..presents.len()).step_by(elf) {
            presents[house] += elf * 10;
        }
    }

    for (housen, giftsn) in presents.iter().enumerate() {
        if *giftsn > 34_000_000 {
            println!("House {} got {} presents", housen, giftsn);
            break;
        }
    }
}

fn lazyelves() {
    let mut deliveries = Vec::with_capacity(34_000_000);
    for i in 0..deliveries.capacity() {
        deliveries.push(1);
    }

    for elf in 2..deliveries.len() {
        let mut i = 0;
        for house in (elf-1..deliveries.len()).step_by(elf) {
            deliveries[house] += elf * 11;
            i += 1;
            if i >= 50 { break; }
        }
        if deliveries[elf - 1] >= 34_000_000 {
            println!("Delivered {} presents to house {}", deliveries[elf - 1], elf);
            return;
        }
    }
}
