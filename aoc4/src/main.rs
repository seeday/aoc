extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let mut sh = Md5::new();
    let mut count = 1000;
    loop {
        sh.input_str(&format!("iwrupvqb{}", count));
        let output = sh.result_str();
        sh.reset();
        if output.starts_with("000000") {
            println!("\n{}{}:{}", "iwripvqb", count, output);
            break;
        }
        if count % 10000 == 0 {
            print!("{}..", count);
        }
        count += 1;
    }
}
