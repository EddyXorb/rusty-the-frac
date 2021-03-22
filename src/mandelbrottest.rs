use crate::complex;

pub struct MandelbrotTestResult {
    pub iterations: u8,
    pub is_in: bool,
}

pub fn is_in_mandelbrot_set(c: complex::Cx, max_count: u8) -> MandelbrotTestResult {
    let mut counter: u8 = 0;
    let mut z = complex::Cx::new(0.0, 0.0);

    while z.abs() < 4.0 && counter < max_count {
        z = (z * z) + c;
        counter += 1;
    }

    MandelbrotTestResult {
        iterations: counter,
        is_in: counter == max_count,
    }
}
