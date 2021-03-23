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

fn main() {
    let image = ImageCreator::new(
        ScreenCoordinates { x: 300, y: 300 },
        Cx { r: 0.0, i: 0.0 },
        Cx { r: 1.0, i: 1.0 },
    )
    .create_image();

    //println!("{:?}", image.rgba());

    start_widget(image);
}
