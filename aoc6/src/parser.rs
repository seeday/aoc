use nom::*;
use std::str::{self, FromStr};

#[derive(Eq, PartialEq, Debug)]
pub enum Op {
    Toggle,
    On,
    Off,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point{x: x, y: y}
    }
}

#[derive(Debug)]
pub struct Tr {
    pub operation: Op,
    pub from: Point,
    pub to: Point,
}

named!(operation<&[u8], Op>, alt!(
        tag!("toggle") => { |_| Op::Toggle} |
        tag!("turn on") => {|_| Op::On} |
        tag!("turn off") => {|_| Op::Off}
        )
    );

named!(tuple<Point>, chain!(
        xx: dbg!(take_until_and_consume!(",")) ~
        yy: take_while!(is_digit),
        || Point{x: u32::from_str(str::from_utf8(xx).unwrap()).unwrap(), y: u32::from_str(str::from_utf8(yy).unwrap()).unwrap()}
        ));

named!(pub full<&[u8], Tr>, chain!(
        op: dbg!(operation) ~
        multispace ~
        from: dbg!(tuple) ~
        tag!(" through ") ~
        to: dbg!(tuple),
        || Tr{operation: op, from: from, to: to}
    ));


#[test]
fn test() {
    assert!(full(b"turn off 370,39 through 425,839").is_done());
    assert!(full(b"turn off 464,858 through 833,915").is_done());
    assert!(full(b"turn off 812,389 through 865,874").is_done());
    assert!(full(b"turn on 599,989 through 806,993").is_done());
    assert!(full(b"turn on 376,415 through 768,548").is_done());
    assert!(full(b"turn on 606,361 through 892,600").is_done());
    assert!(full(b"turn off 448,208 through 645,684").is_done());
    assert!(full(b"toggle 50,472 through 452,788").is_done());
    assert!(full(b"toggle 205,417 through 703,826").is_done());
    assert!(full(b"toggle 533,331 through 906,873").is_done());
    assert!(full(b"toggle 857,493 through 989,970").is_done());
    assert!(full(b"turn off 631,950 through 894,975").is_done());
    assert!(full(b"turn off 387,19 through 720,700").is_done());
    assert!(full(b"turn off 511,843 through 581,945").is_done());
    assert!(full(b"toggle 514,557 through 662,883").is_done());
    assert!(full(b"turn off 269,809 through 876,847").is_done());
    assert!(full(b"turn off 149,517 through 716,777").is_done());
    assert!(full(b"turn off 994,939 through 998,988").is_done());
    assert!(full(b"toggle 467,662 through 555,957").is_done());
    assert!(full(b"turn on 952,417 through 954,845").is_done());
    assert!(full(b"turn on 565,226 through 944,880").is_done());
    assert!(full(b"turn on 214,319 through 805,722").is_done());
    assert!(full(b"toggle 532,276 through 636,847").is_done());
    assert!(full(b"toggle 619,80 through 689,507").is_done());
    assert!(full(b"turn on 390,706 through 884,722").is_done());
    assert!(full(b"toggle 17,634 through 537,766").is_done());
    assert!(full(b"toggle 706,440 through 834,441").is_done());
    assert!(full(b"toggle 318,207 through 499,530").is_done());
    assert!(full(b"toggle 698,185 through 830,343").is_done());
    assert!(full(b"toggle 566,679 through 744,716").is_done());
    assert!(full(b"toggle 347,482 through 959,482").is_done());
    assert!(full(b"toggle 39,799 through 981,872").is_done());
    assert!(full(b"turn on 583,543 through 846,710").is_done());
    assert!(full(b"turn off 367,664 through 595,872").is_done());
}

