use druid::Data;
use std::{ptr, vec};

use crate::screencoordinates::ScreenCoordinates;
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Saves raw data of image in order to be easily painted by widgets.
#[derive(Clone)]
pub struct RawImage {
    rgba: Vec<u8>,
    height: usize,
    width: usize,
}

impl Data for RawImage {
    fn same(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl RawImage {
    fn xy_to_vec_index(&self, coords: &ScreenCoordinates) -> usize {
        coords.y * self.width + coords.x
    }

    pub fn insert(&mut self, coords: &ScreenCoordinates, colour: &RGBA) {
        let start = self.xy_to_vec_index(coords);
        self.rgba[start] = colour.r;
        self.rgba[start + 1] = colour.g;
        self.rgba[start + 2] = colour.b;
        self.rgba[start + 3] = colour.a;
    }

    pub fn new(screensize: ScreenCoordinates) -> Self {
        Self {
            rgba: vec![0; screensize.y * screensize.x * 4],
            height: screensize.y,
            width: screensize.x,
        }
    }

    /// Get a reference to the mandelbrot image's rgba.
    pub fn rgba(&self) -> &Vec<u8> {
        &self.rgba
    }

    /// Get a reference to the mandelbrot image's height.
    pub fn height(&self) -> &usize {
        &self.height
    }

    /// Set the mandelbrot image's width.
    pub fn width(&self) -> &usize {
        &self.width
    }
}
