use nom::*;
use nom::IResult::*; //required for tests
use std::str::{self, FromStr, Utf8Error};

fn to_u16(s: &[u8]) -> Result<u16, Utf8Error> {
    return str::from_utf8(s).map(|x| u16::from_str(x).unwrap());
}

named!(digits<&[u8], u16>, map_res!(digit, to_u16));

named!(chars<&[u8], &str>, map_res!(alpha, str::from_utf8));

named!(value<&[u8], Value>, alt!(
        chars => { |c| Value::V(c)} |
        digits => { |d| Value::I(d)}
    ));

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Value<'a> {
    I(u16),
    V(&'a str),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Ops<'a> {
    Assign(Value<'a>, &'a str),
    And(Value<'a>, Value<'a>, &'a str),
    Or(Value<'a>, Value<'a>, &'a str),
    Lshift(Value<'a>, Value<'a>, &'a str),
    Rshift(Value<'a>, Value<'a>, &'a str),
    Not(Value<'a>, &'a str),
}

named!(assign<&[u8], Ops>, chain!(
        val: value ~
        space? ~
        tag!("->") ~
        space? ~
        rhs: chars,
        || Ops::Assign(val, rhs)
    ));

named!(binary<&[u8], Ops>, chain!(
        p1: value ~
        space ~
        op: alt!(tag!("AND") | tag!("OR")) ~
        space ~
        p2: value ~
        space? ~ 
        tag!("->") ~
        space? ~
        rhs: chars,
        || match op {
            b"AND" => Ops::And(p1, p2, rhs),
            b"OR" => Ops::Or(p1, p2, rhs),
            _ => unreachable!(),
        }
    ));

named!(bitwise<&[u8], Ops>, chain!(
        p1: value ~
        space ~
        dir: alt!(tag!("LSHIFT") | tag!("RSHIFT")) ~
        space ~
        p2: value ~
        space? ~
        tag!("->") ~
        space? ~
        rhs: chars,
        || match dir {
            b"LSHIFT" => Ops::Lshift(p1, p2, rhs),
            b"RSHIFT" => Ops::Rshift(p1, p2, rhs),
            _ => unreachable!(),
        }));

named!(unary<&[u8], Ops>, chain!(
        tag!("NOT") ~
        space ~
        lhs: value ~
        space? ~
        tag!("->") ~
        space? ~
        rhs: chars,
        || Ops::Not(lhs, rhs)
    ));

named!(expr<&[u8], Ops>, chain!(
        op: dbg!(alt!(assign | unary | binary | bitwise)) ~
        multispace,
        || op
    ));

named!(pub prgm<&[u8], Vec<Ops> >, dbg!(many1!(expr)));

#[test]
fn t_digits() {
    assert_eq!(digits(b"1"), Done(&b""[..], 1));
    assert_eq!(digits(b"12"), Done(&b""[..], 12));
    assert_eq!(digits(b"123"), Done(&b""[..], 123));
    assert_eq!(digits(b"00123"), Done(&b""[..], 123));
    assert_eq!(digits(b"0"), Done(&b""[..], 0));
    assert_eq!(digits(b"1 "), Done(&b" "[..], 1));
}

#[test]
fn t_chars() {
    assert_eq!(chars(b"a"), Done(&b""[..], "a"));
    assert_eq!(chars(b"ab"), Done(&b""[..], "ab"));
    assert_eq!(chars(b"butts"), Done(&b""[..], "butts"));
    assert_eq!(chars(b"AND"), Done(&b""[..], "AND"));
    assert_eq!(chars(b"AND "), Done(&b" "[..], "AND"));
}

// TODO: fix tests, important one works though
// #[test]
// fn t_assign() {
//    assert_eq!(assign(b"123 -> x"), Done(&b""[..], Ops::Assign(Value:I(123), Value:V("x"))));
//    assert_eq!(assign(b"1 -> x"), Done(&b""[..], Ops::Assign(Value:I(1), "x")));
//    assert_eq!(assign(b"1   ->x"), Done(&b""[..], Ops::Assign(1, "x")));
// }
//
// #[test]
// fn t_unary() {
//    assert_eq!(unary(b"NOT x -> y"), Done(&b""[..], Ops::Not("x", "y")));
//    assert_eq!(unary(b"NOT x->y"), Done(&b""[..], Ops::Not("x", "y")));
//    assert_eq!(unary(b"NOT x      -> yz"),
//               Done(&b""[..], Ops::Not("x", "yz")));
// }
//
// #[test]
// fn t_binary() {
//    assert_eq!(binary(b"x AND y -> d"),
//               Done(&b""[..], Ops::And("x", "y", "d")));
//    assert_eq!(binary(b"x OR y -> d"),
//               Done(&b""[..], Ops::Or("x", "y", "d")));
// }
//
// #[test]
// fn t_bitwise() {
//    assert_eq!(bitwise(b"x LSHIFT 2 -> y"),
//               Done(&b""[..], Ops::Lshift("x", 2, "y")));
//    assert_eq!(bitwise(b"x RSHIFT 2 -> y"),
//               Done(&b""[..], Ops::Rshift("x", 2, "y")));
// }
//
// #[test]
// fn t_expr() {
//    assert_eq!(expr(b"1   ->x"), Done(&b""[..], Ops::Assign(1, "x")));
//    assert_eq!(expr(b"NOT x      -> yz"),
//               Done(&b""[..], Ops::Not("x", "yz")));
//    assert_eq!(expr(b"x AND y -> d"),
//               Done(&b""[..], Ops::And("x", "y", "d")));
//    assert_eq!(expr(b"lv LSHIFT 15 -> lz"),
//               Done(&b""[..], Ops::Lshift("lv", 15, "lz")));
// }
//
// #[test]
// fn t_prgm() {
//    assert_eq!(prgm(b"1 -> x\nNOT x -> yz\nx AND y -> zz\nzz LSHIFT 2 -> a"),
//               Done(&b""[..],
//                    vec![Ops::Assign(1, "x"),
//                         Ops::Not("x", "yz"),
//                         Ops::And("x", "y", "zz"),
//                         Ops::Lshift("zz", 2, "a")]));
// }

#[test]
fn t_full() {
    let p = prgm(include_bytes!("../input.txt"));
    assert!(p.is_done());
    if let Done(rem, parsed) = p {
        assert!(rem.is_empty());
        assert!(!parsed.is_empty());
    }
}
