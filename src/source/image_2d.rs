use super::Source2D;
use crate::pixel::Pixel;

/// A 2D image.
pub struct Image2D<T> {
    /// Pixels of an object, flattened into a 1D vector
    pixels: Vec<T>,
    /// Width of image in pixels
    width: usize,
    /// Height of image in pixels
    height: usize,
}

impl<T> Source2D<T> for Image2D<T>
where
    T: Pixel + Default,
{
    fn get_pixel(&self, x: usize, y: usize) -> &T {
        &self.pixels[y * self.width + x]
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_width(&self) -> usize {
        self.width
    }
}
