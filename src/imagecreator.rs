use crate::{
    complex::Cx, mandelbrottest, rawimage::RGBA, screencoordinates::ScreenCoordinates,
    transformer::Transformer, RawImage,
};
/// Creates rawimage of mandelbrotset with zoom und center
pub struct ImageCreator {
    center: Cx,
    dims: Cx,
    screensize: ScreenCoordinates,
}

impl ImageCreator {
    pub fn new(screensize: ScreenCoordinates, center: Cx, dims: Cx) -> Self {
        ImageCreator {
            center,
            dims,
            screensize,
        }
    }

    pub fn createImage(self) -> RawImage {
        let mut out = RawImage::new(self.screensize);
        let converter = Transformer::new(self.screensize, self.dims, self.center);
        for x in 0..self.screensize.x {
            for y in 0..self.screensize.y {
                let coords = ScreenCoordinates { x, y };
                let cx_coords = converter.toCx(coords);
                let result = mandelbrottest::is_in_mandelbrot_set(cx_coords, 100);
                let rgba = RGBA {
                    r: result.iterations,
                    g: result.iterations,
                    b: result.iterations,
                    a: 255,
                };

                out.insert(&coords, &rgba);
            }
        }
        out
    }
}
