use crate::complex::Cx;
use std::ops::{Mul, Sub};

#[derive(Copy, Clone)]
pub struct ScreenCoordinates {
    pub x: usize,
    pub y: usize,
}

impl Mul<f64> for ScreenCoordinates {
    type Output = ScreenCoordinates;

    fn mul(self, rhs: f64) -> ScreenCoordinates {
        ScreenCoordinates {
            x: self.x * rhs as usize,
            y: self.y * rhs as usize,
        }
    }
}

impl Sub for ScreenCoordinates {
    type Output = ScreenCoordinates;

    fn sub(self, rhs: ScreenCoordinates) -> Self::Output {
        ScreenCoordinates {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
