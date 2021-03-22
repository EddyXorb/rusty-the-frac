use mandelbrotimage::MandelbrotImage;

mod complex;
mod mandelbrotimage;
mod mandelbrottest;
mod widget;

fn make_image_data(width: usize, height: usize) -> Vec<u8> {
    let mut result = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let ix = (y * width + x) * 4;
            result[ix] = x as u8;
            result[ix + 1] = y as u8;
            result[ix + 2] = !(x as u8);
            result[ix + 3] = 200;
        }
    }
    result
}

fn main() {
    let width = 124;
    let height = 124;
    let data = MandelbrotImage {
        rgba: make_image_data(width, height),
        width,
        height,
    };
    widget::start_widget(data);

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
