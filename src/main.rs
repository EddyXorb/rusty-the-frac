use complex::Cx;
use imagecreator::ImageCreator;
use rawimage::RawImage;
use screencoordinates::ScreenCoordinates;
use widget::start_widget;

mod complex;
mod imagecreator;
mod mandelbrottest;
mod rawimage;
mod screencoordinates;
mod transformer;
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
    let image = ImageCreator::new(
        ScreenCoordinates { x: 512, y: 512 },
        Cx { r: 0.0, i: 0.0 },
        Cx { r: 2.0, i: 2.0 },
    )
    .createImage();

    start_widget(image);
}
