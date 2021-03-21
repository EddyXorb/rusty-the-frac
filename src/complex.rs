use std::{
    fmt::Display,
    ops::{Add, Mul},
};

#[derive(Copy, Clone)]
pub struct Cx {
    pub r: f64,
    pub i: f64,
}

impl Cx {
    pub fn new(r: f64, i: f64) -> Self {
        Self { r, i }
    }

    pub fn abs(self) -> f64 {
        self.r * self.r + self.i * self.i
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
