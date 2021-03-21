use std::{
    fmt::Display,
    ops::{Add, Mul},
};

pub struct Cx {
    pub r: i64,
    pub i: i64,
}

impl Cx {
    pub fn new(r: i64, i: i64) -> Self {
        Self { r, i }
    }
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

impl Mul for Cx {
    type Output = Cx;

    fn mul(self, rhs: Cx) -> Self::Output {
        {
            Cx {
                r: self.r * rhs.r - self.i * rhs.i,
                i: self.r * rhs.i + self.i * rhs.r,
            }
        }
    }
}

impl Display for Cx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.r, self.i)
    }
}
