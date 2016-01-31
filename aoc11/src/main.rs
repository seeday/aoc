struct Letters {
    // limits are 97-122, skipping 105(i), 108(l), 111(o)
    pub l: Vec<u8>,
}

impl Letters {
    fn new(s: String) -> Letters {
        Letters { l: s.into_bytes() }
    }

    fn increment(&mut self) {
        let mut carry = true;
        for i in (0..(self.l.len())).rev() {
            let mut x = self.l[i];
            if carry {
                x += 1;
                carry = false;

                if x + 1 > 123 {
                    x -= 26;
                    carry = true;
                }
                if x == 105 || x == 108 || x == 111 {
                    x += 1;
                }
                self.l[i] = x;
            }
        }

        if carry {
            self.l[0] -= 26;
            self.l.insert(0, 97);
        }
    }

    fn test(&self) -> bool {
        let mut pair1 = false;
        let mut let1 = 0;
        let mut pair2 = false;
        let mut let2 = 0;
        let mut inc = 0;
        let mut linc = 0;
        for i in self.l.iter() {
            if !pair1 {
                if let1 == *i {
                    pair1 = true;
                } else {
                    let1 = *i;
                }
            } else if !pair2 {
                if let2 == *i {
                    pair2 = true;
                } else {
                    let2 = *i;
                }
            }
            if linc != 3 {
                if *i - 1 == inc {
                    inc = *i;
                    linc += 1;
                } else {
                    inc = *i;
                    linc = 1;
                }
            }
        }

        pair1 && pair2 && linc == 3
    }

    fn as_string(&self) -> String {
        String::from_utf8(self.l.clone()).unwrap()
    }
}

fn main() {
    let mut l = Letters::new("cqjxjnds".to_string());
    loop {
        l.increment();
        if l.test() {
            println!("{}\t{:?}", l.as_string(), l.l);
        }
    }
}
