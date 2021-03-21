mod complex;
mod mandelbrottest;

fn main() {
    let a = complex::Cx::new(1.0, 1.0);
    let b = complex::Cx::new(0.0, 10.0);
    let c = a * b;

    println!("{}", c);
    println!(
        "{} is in set? {}",
        c,
        mandelbrottest::is_in_mandelbrot_set(c)
    );
}
