use druid::platform_menus::win::file::new;

use crate::complex::Cx;

pub struct ScreenCoordinates {
    pub x: usize,
    pub y: usize,
}
struct CxToScreenConverter {
    target_h: usize,
    target_w: usize,
    cx_max_r: usize,
    cx_max_i: usize,
    cent_x: usize,
    cent_y: usize,
}

impl CxToScreenConverter {
    pub fn new(target_h: usize, target_w: usize, cx_max_r: usize, cx_max_i: usize) {
        CxToScreenConverter {
            target_h,
            target_w,
            cx_max_r,
            cx_max_i,
            cent_x: target_w / 2,
            cent_y: target_h / 2,
        };
    }

    pub fn toScreen(self, c: Cx) -> ScreenCoordinates {
        let x = (c.r / self.cx_max_r) * self.cent_x + self.cent_x as usize;
        let y = (c.i / self.cx_max_i) * self.cent_y + self.cent_y as usize;
        ScreenCoordinates { x, y }
    }

    pub fn toCx(self, s: ScreenCoordinates) -> Cx {
        let r = self.cx_max_r * (s.x - self.cent_x) / self.cent_x;
        let i = self.cx_max_i * (s.y - self.cent_y) / self.cent_y;
        Cx { r, i }
    }
}
