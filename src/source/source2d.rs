use crate::pixel;

pub trait Source2D {
    type Pixel: pixel::Pixel + Default;
    fn get_pixel(&self, x: usize, y: usize) -> Self::Pixel;
}

