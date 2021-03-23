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
            center: ScreenCoordinates {
                x: max.x / 2,
                y: max.y / 2,
            },
            cx_max,
            cx_center,
        }
    }

    pub fn to_screen(self, c: Cx) -> ScreenCoordinates {
        let centered_c = c - self.cx_center;
        let x = ((centered_c.r / self.cx_max.r) * (self.max.x as f64)) as i32 + self.center.x;
        let y = ((centered_c.i / self.cx_max.i) * (self.max.y as f64)) as i32 + self.center.y;

        ScreenCoordinates { x, y }
    }

    pub fn to_cx(&self, s: ScreenCoordinates) -> Cx {
        let diff = s - self.center;
        let dx = diff.x as f64;
        let dy = diff.y as f64;
        let r = self.cx_max.r * dx / (self.max.x as f64) + self.cx_center.r;
        let i = self.cx_max.i * dy / (self.max.y as f64) + self.cx_center.i;

        Cx { r, i }
    }
}
