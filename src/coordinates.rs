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
pub struct CxToScreenConverter {
    max: ScreenCoordinates,
    center: ScreenCoordinates,
    cx_max: Cx,
    cx_center: Cx,
}

impl CxToScreenConverter {
    pub fn new(max: ScreenCoordinates, cx_max: Cx, cx_center: Cx) {
        CxToScreenConverter {
            max,
            center: max * 0.5,
            cx_max,
            cx_center,
        };
    }

    pub fn toScreen(self, c: Cx) -> ScreenCoordinates {
        let centered_c = c - self.cx_center;
        let x = ((centered_c.r / self.cx_max.r) * (self.center.x as f64)) as usize + self.center.x;
        let y = ((centered_c.i / self.cx_max.i) * (self.center.y as f64)) as usize + self.center.y;

        ScreenCoordinates { x, y }
    }

    pub fn toCx(self, s: ScreenCoordinates) -> Cx {
        let diff = s - self.center;
        let dx = diff.x as f64;
        let dy = diff.y as f64;
        let r = self.cx_max.r * dx / (self.center.x as f64) + self.cx_center.r;
        let i = self.cx_max.i * dy / (self.center.y as f64) + self.cx_center.i;

        Cx { r, i }
    }
}
