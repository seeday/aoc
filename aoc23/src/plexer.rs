use nom::*;
use std::str::{self, FromStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Reg {
    A,
    B
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Ins {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(i8),
    Jie(Reg, i8),
    Jio(Reg, i8)
}

named!(num<i8>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(int<i8>, delimited!(opt!(multispace), alt!(
        preceded!(tag!("+"), num) => {|x:i8|  x} |
        preceded!(tag!("-"), num) => {|x:i8| -x}
    ), opt!(multispace)));

named!(reg<Reg>, delimited!(opt!(multispace), 
        alt!(tag!("a") => {|_| Reg::A}|
             tag!("b") => {|_| Reg::B})
    , opt!(multispace)));

named!(hlf<Ins>, chain!(
        tag!("hlf") ~
        multispace? ~
        r: reg,
        || Ins::Hlf(r)
    ));

named!(tpl<Ins>, chain!(
        tag!("tpl") ~
        multispace? ~
        r: reg,
        || Ins::Tpl(r)
    ));

named!(inc<Ins>, chain!(
        tag!("inc") ~
        multispace? ~
        r: reg,
        || Ins::Inc(r)
    ));

named!(jmp<Ins>, chain!(
        tag!("jmp") ~
        multispace? ~
        o: int,
        || Ins::Jmp(o)
    ));

named!(jie<Ins>, chain!(
        tag!("jie") ~
        multispace? ~
        r: reg ~
        tag!(",") ~
        multispace? ~
        o: int,
        || Ins::Jie(r, o)
    ));

named!(jio<Ins>, chain!(
        tag!("jio") ~
        multispace? ~
        r: reg ~
        tag!(",") ~
        multispace? ~
        o: int,
        || Ins::Jio(r, o)
    ));

named!(expr<Ins>, chain!(
        e: alt!(hlf| tpl| inc| jmp| jie| jio) ~
        multispace?,
        || e
    ));

named!(pub prgm<Vec<Ins> >, many1!(expr));

#[test]
fn t_int() {
    assert_eq!(int(b"+5"), IResult::Done(&b""[..], 5));
    assert_eq!(int(b"-5"), IResult::Done(&b""[..], -5));
    assert_eq!(int(b" -5, "), IResult::Done(&b", "[..], -5));
}

#[test]
fn t_reg() {
    assert_eq!(reg(b"a"), IResult::Done(&b""[..], Reg::A));
    assert_eq!(reg(b"b"), IResult::Done(&b""[..], Reg::B));
    assert_eq!(reg(b" b "), IResult::Done(&b""[..], Reg::B));
    assert_eq!(reg(b" b, "), IResult::Done(&b", "[..], Reg::B));
}

#[test]
fn t_expr() {
    assert_eq!(expr(b"hlf a"), IResult::Done(&b""[..], Ins::Hlf(Reg::A)));
    assert_eq!(expr(b"hlf b"), IResult::Done(&b""[..], Ins::Hlf(Reg::B)));
    assert_eq!(expr(b"tpl b"), IResult::Done(&b""[..], Ins::Tpl(Reg::B)));
    assert_eq!(expr(b"inc a"), IResult::Done(&b""[..], Ins::Inc(Reg::A)));
    assert_eq!(expr(b"jmp +5"), IResult::Done(&b""[..], Ins::Jmp(5)));
    assert_eq!(expr(b"jmp -5"), IResult::Done(&b""[..], Ins::Jmp(-5)));
    assert_eq!(expr(b"jie a, +5"), IResult::Done(&b""[..], Ins::Jie(Reg::A, 5)));
    assert_eq!(expr(b"jie b, -5"), IResult::Done(&b""[..], Ins::Jie(Reg::B, -5)));
    assert_eq!(expr(b"jio a, +23"), IResult::Done(&b""[..], Ins::Jio(Reg::A, 23)));
}

#[test]
fn t_prgm() {
    assert_eq!(prgm(b"hlf a\nhlf b\njmp -5\njie a, +5\n"), IResult::Done(&b""[..], vec![Ins::Hlf(Reg::A), Ins::Hlf(Reg::B), Ins::Jmp(-5), Ins::Jie(Reg::A, 5)]));
}
