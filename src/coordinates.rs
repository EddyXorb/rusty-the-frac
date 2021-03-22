use crate::complex::Cx;
use druid::platform_menus::win::file::new;
use std::ops::Mul;

pub struct ScreenCoordinates {
    pub x: usize,
    pub y: usize,
}

impl Mul for ScreenCoordinates {
    type Output = &ScreenCoordinates;

    fn mul(self, rhs: f64) -> &Self::Output {
        self.x = self.x * rhs as usize;
        self.y = self.y * rhs as usize;
    }
}
struct CxToScreenConverter {
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
        let x = (centered_c.r / self.cx_max.r) * self.center.x + self.center.x as usize;
        let y = (centered_c.i / self.cx_max.i) * self.center.y + self.center.y as usize;

        ScreenCoordinates { x, y }
    }

    pub fn toCx(self, s: ScreenCoordinates) -> Cx {
        let r = self.cx_max.r * (s.x - self.center.x) / self.center.x + self.cx_center.r;
        let i = self.cx_max.i * (s.y - self.center.y) / self.center.y + self.cx_center.i;

        Cx { r, i }
    }
}
