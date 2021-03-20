use std::ops::Add;

struct Cx {
    r: i64,
    i: i64,
}

impl Add for Cx {
    type Output = Cx;

    fn add(self, rhs: Cx) -> Self::Output {
        Cx {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl Cx {
    fn new(r: i64, i: i64) -> Self {
        Self { r, i }
    }
}

fn main() {
    println!("Hello, world!");
}
