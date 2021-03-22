use druid::Data;
use std::ptr;

#[derive(Clone)]
pub struct MandelbrotImage {
    pub rgba: Vec<u8>,
    pub height: usize,
    pub width: usize,
}

impl Data for MandelbrotImage {
    fn same(&self, other: &Self) -> bool {
        self.height == other.height && self.width == other.width && ptr::eq(&self.rgba, &other.rgba)
    }
}
