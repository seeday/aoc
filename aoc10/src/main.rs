#![feature(str_char)]

fn las(input: String) -> String {
    let mut output = String::with_capacity(input.len());
   
    let mut cc = input.char_at(0);
    let mut count = 1;
    for c in input.chars().skip(1) {
        if c == cc {
            count += 1;
        } else {
            output.push_str(&format!("{}{}", count, cc));
            cc = c;
            count = 1;
        }
    }
    output.push_str(&format!("{}{}", count, cc));
    output
}

fn main() {
    let mut string = String::with_capacity(100);
    string.push_str("1321131112");
    for i in 0..50 {
        string = las(string);
        println!("{}: {:>7}", i+1, string.len());
    }
}
