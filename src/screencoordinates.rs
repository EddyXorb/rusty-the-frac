use std::{
    fmt::Display,
    ops::{Mul, Sub},
};

#[derive(Copy, Clone)]
pub struct ScreenCoordinates {
    pub x: i32,
    pub y: i32,
}

impl Mul<f64> for ScreenCoordinates {
    type Output = ScreenCoordinates;

    fn mul(self, rhs: f64) -> ScreenCoordinates {
        ScreenCoordinates {
            x: self.x * rhs as i32,
            y: self.y * rhs as i32,
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

impl Display for ScreenCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
