use crate::complex::Cx;
use crate::screencoordinates::ScreenCoordinates;

pub struct Transformer {
    max: ScreenCoordinates,
    center: ScreenCoordinates,
    cx_max: Cx,
    cx_center: Cx,
}

impl Transformer {
    pub fn new(max: ScreenCoordinates, cx_max: Cx, cx_center: Cx) -> Transformer {
        Transformer {
            max,
            center: max * 0.5,
            cx_max,
            cx_center,
        }
    }

    pub fn toScreen(self, c: Cx) -> ScreenCoordinates {
        let centered_c = c - self.cx_center;
        let x = ((centered_c.r / self.cx_max.r) * (self.center.x as f64)) as usize + self.center.x;
        let y = ((centered_c.i / self.cx_max.i) * (self.center.y as f64)) as usize + self.center.y;

        ScreenCoordinates { x, y }
    }

    pub fn toCx(&self, s: ScreenCoordinates) -> Cx {
        let diff = s - self.center;
        let dx = diff.x as f64;
        let dy = diff.y as f64;
        let r = self.cx_max.r * dx / (self.center.x as f64) + self.cx_center.r;
        let i = self.cx_max.i * dy / (self.center.y as f64) + self.cx_center.i;

        Cx { r, i }
    }
}
