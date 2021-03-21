use crate::complex;

pub fn is_in_mandelbrot_set(c: complex::Cx) -> bool {
    let mut counter: i8 = 0;
    let mut z = complex::Cx::new(0.0, 0.0);

    while z.abs() < 4.0 && counter < 100 {
        z = (z * z) + c;
        counter += 1;
    }

    counter >= 100
}
