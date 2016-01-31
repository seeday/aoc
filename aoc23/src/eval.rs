use plexer::*;
use nom::IResult;

macro_rules! uadd {
    ($x:expr, $y:expr) => {{
        if $y < 0 { $x -= -$y as u16 } else { $x += $y as u16 }
    }};
}

#[derive(Debug)]
pub struct Eval {
    pc: u16,
    a: u32,
    b: u32,
    prgm: Vec<Ins>,
}

impl Eval {
    pub fn load(pro: String) -> Eval {
        if let IResult::Done(rem, prgm) = prgm(pro.as_bytes()) {
            assert!(rem.is_empty());
            Eval{pc: 0, a: 1, b: 0, prgm: prgm}
        } else {
            panic!("Couldn't parse program");
        }
    }

    fn reg(&mut self, r: Reg) -> &mut u32 {
        match r {
            Reg::A => &mut self.a,
            Reg::B => &mut self.b
        }
    }

    fn step(&mut self) {
        let ins = self.prgm[self.pc as usize];
        match ins {
            Ins::Hlf(r) => { *self.reg(r) /= 2; self.pc += 1; },
            Ins::Tpl(r) => { *self.reg(r) *= 3; self.pc += 1; },
            Ins::Inc(r) => { *self.reg(r) += 1; self.pc += 1; },
            Ins::Jmp(o) => uadd!(self.pc, o),
            Ins::Jie(r, o) => if *self.reg(r) % 2 == 0 { uadd!(self.pc ,o) } else { self.pc += 1 },
            Ins::Jio(r, o) => if *self.reg(r) == 1 { uadd!(self.pc, o) } else { self.pc += 1 },
        }
    }

    pub fn run(&mut self) {
        while (self.pc as usize) < self.prgm.len() {
            self.step();
        }
    }
}

