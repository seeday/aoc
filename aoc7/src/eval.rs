use std::collections::HashMap;
use std::str;
use nom::IResult::*;
use plexer::*;

pub struct Eval<'a> {
    // since there is only one solution to each program, we don't have to worry about values
    // changing and cascading changes. We'd just completely reload the program. Therefore, a static
    // environment can cache everything solved for, and eval can just add new things to the env.
    rules: HashMap<&'a str, Ops<'a>>,
    pub env: HashMap<&'a str, u16>,
}

impl<'a> Eval<'a> {
    pub fn new() -> Eval<'a> {
        Eval {
            rules: HashMap::with_capacity(100),
            env: HashMap::with_capacity(100),
        }
    }

    pub fn load(&mut self, file: &'a [u8]) {
        if let Done(remaining, parsed) = prgm(file) {
            if remaining.len() != 0 {
                panic!("Didn't fully parse file!\n{:?}", str::from_utf8(remaining));
            }
            for op in parsed.into_iter() {
                self.rules.insert(match op {
                                      Ops::Assign(_, n) => n,
                                      Ops::And(_, _, n) => n,
                                      Ops::Or(_, _, n) => n,
                                      Ops::Lshift(_, _, n) => n,
                                      Ops::Rshift(_, _, n) => n,
                                      Ops::Not(_, n) => n,
                                  },
                                  op);
            }
        }
    }

    fn veval(&mut self, val: Value<'a>) -> u16 {
        match val {
            Value::I(a) => a,
            Value::V(a) => self.eval(a),
        }
    }

    pub fn eval(&mut self, val: &'a str) -> u16 {
        if let Some(res) = self.env.get(val) {
            return *res;
        }
        let op = *self.rules
                      .get_mut(val)
                      .expect(format!("Value {} not found in rules!", val).as_str());
        match op {
            Ops::Assign(a, _) => {
                let x = self.veval(a);
                self.env.insert(val, x);
                return x;
            }
            Ops::And(a, b, _) => {
                let x = self.veval(a);
                let y = self.veval(b);
                let r = x & y;
                self.env.insert(val, r);
                return r;
            }
            Ops::Or(a, b, _) => {
                let x = self.veval(a);
                let y = self.veval(b);
                let r = x | y;
                self.env.insert(val, r);
                return r;
            }
            Ops::Lshift(a, b, _) => {
                let x = self.veval(a);
                let y = self.veval(b);
                let r = x << y;
                self.env.insert(val, r);
                return r;
            }
            Ops::Rshift(a, b, _) => {
                let x = self.veval(a);
                let y = self.veval(b);
                let r = x >> y;
                self.env.insert(val, r);
                return r;
            }
            Ops::Not(a, _) => {
                let x = !self.veval(a);
                self.env.insert(val, x);
                return x;
            }
        }
    }

    pub fn solve(&mut self) {
        for k in self.rules.clone().keys() {
            self.eval(k);
        }
    }

    pub fn dumpenv(&self) {
        println!("env: {{");
        let mut vec = self.env.clone().into_iter().collect::<Vec<(&str, u16)>>();
        vec.sort();
        for (k, v) in vec {
            println!("    \"{}\": {}", k, v);
        }
        println!("}}");
    }
}

#[test]
fn t_load() {
    let mut e = Eval::new();
    e.load(include_bytes!("../test.txt"));
}

#[test]
fn t_eval() {
    let mut e = Eval::new();
    e.load(include_bytes!("../test.txt"));
    assert_eq!(e.eval("x"), 123);
    assert_eq!(e.eval("y"), 456);
    e.rules.remove("y");
    assert_eq!(e.eval("y"), 456); //env lookup
    assert_eq!(e.eval("d"), 123 & 456);
    e.rules.remove("d");
    assert_eq!(e.eval("d"), 123 & 456); //test env lookup
    assert_eq!(e.eval("e"), 123 | 456);
    assert_eq!(e.eval("f"), 123 << 2);
    assert_eq!(e.eval("g"), 456 >> 2);
    assert_eq!(e.eval("h"), !123);
}

#[test]
#[should_panic(expected = "Value asdf not found in rules!")]
fn t_missing() {
    let mut e = Eval::new();
    e.load(include_bytes!("../test.txt"));
    e.eval("asdf");
}
