use super::Source2D;
use crate::pixel::Pixel;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq)]
/// A 2D image.
pub struct Image2D<P, Container> {
    /// Pixels of an object, flattened into a 1D vector
    pixels: Container,
    /// Width of image in pixels
    width: usize,
    /// Height of image in pixels
    height: usize,
    _phantom: PhantomData<P>,
}

impl<P, Container> Image2D<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P]>,
{
    /// Create a new image from a vector of pixels.
    /// Panics at runtime if the bounds are not satisfied.
    ///
    /// # Bounds
    ///
    /// `width > 0`
    ///
    /// `height > 0`
    ///
    /// `width * height == pixels.len()`
    pub fn new(pixels: Container, width: usize, height: usize) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        debug_assert!(width * height == pixels.len());
        Self {
            pixels,
            width,
            height,
            _phantom: PhantomData,
        }
    }

    #[inline]
    /// Map 2D coordinates to a 1D coordinate
    pub fn flatten_coord(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    #[inline]
    /// Map 1D coorinate to a 2D coordinate
    /// Should return `(row, column)`
    pub fn expand_coord(&self, coord: usize) -> (usize, usize) {
        (coord / self.width, coord % self.width)
    }
}

impl<P, Container> Source2D<P> for Image2D<P, Container>
where
    P: Pixel + Default,
    Container: Deref<Target = [P]> + DerefMut,
{
    #[inline]
    /// Set a pixel
    ///
    /// # Example
    /// ```
    /// use sahara::source::{Source2D, Image2D};
    /// use sahara::pixel::RgbaPixel;
    ///
    /// let bl = RgbaPixel::new(0, 0, 0, 255);  // Black pixel
    /// let r = RgbaPixel::new(255, 0, 0, 255); // Red pixel
    ///
    /// // Create a 2 by 2 image that is all black
    /// let mut image_2d = Image2D::new(vec![bl; 4], 2, 2);
    /// image_2d.set_pixel(0, 1, r);
    ///
    /// assert_eq!(image_2d.get_pixel(0, 0), &bl); // Top left corner
    /// assert_eq!(image_2d.get_pixel(0, 1), &r);  // Top right corner
    /// assert_eq!(image_2d.get_pixel(1, 0), &bl); // Bottom left corner
    /// assert_eq!(image_2d.get_pixel(1, 1), &bl); // Bottom right corner

    /// ```
    fn set_pixel(&mut self, row: usize, col: usize, pixel: P) {
        let pos_1d = self.flatten_coord(row, col);
        self.pixels[pos_1d] = pixel;
    }

    #[inline]
    /// Get a pixel
    ///
    /// # Example
    /// ```
    /// use sahara::source::{Source2D, Image2D};
    /// use sahara::pixel::RgbaPixel;
    ///
    /// let bl = RgbaPixel::new(0, 0, 0, 255);  // Black pixel
    /// let r = RgbaPixel::new(255, 0, 0, 255); // Red pixel
    /// let g = RgbaPixel::new(0, 255, 0, 255); // Green pixel
    ///
    /// // Create a 2 by 2 image
    /// let image_2d = Image2D::new(vec![bl, r, g, r], 2, 2);
    ///
    /// assert_eq!(image_2d.get_pixel(0, 0), &bl); // Top left corner
    /// assert_eq!(image_2d.get_pixel(0, 1), &r);  // Top right corner
    /// assert_eq!(image_2d.get_pixel(1, 0), &g);  // Bottom left corner
    /// assert_eq!(image_2d.get_pixel(1, 1), &r);  // Bottom right corner
    /// ```
    fn get_pixel(&self, row: usize, col: usize) -> &P {
        &self.pixels[self.flatten_coord(row, col)]
    }

    #[inline]
    /// Get the height
    ///
    /// # Example
    /// ```
    /// use sahara::source::{Source2D, Image2D};
    /// use sahara::pixel::RgbaPixel;
    ///
    /// let b = RgbaPixel::new(0, 0, 0, 255); // Black Pixel
    ///
    /// let image = Image2D::new(vec![b, b, b, b, b, b], 2, 3);
    ///
    /// assert_eq!(image.get_height(), 3);
    /// ```
    fn get_height(&self) -> usize {
        self.height
    }

    #[inline]
    /// Get the width
    ///
    /// # Example
    /// ```
    /// use sahara::source::{Source2D, Image2D};
    /// use sahara::pixel::RgbaPixel;
    ///
    /// let b = RgbaPixel::from((0, 0, 0, 255)); // Black Pixel
    ///
    /// let image = Image2D::new(vec![b, b, b, b, b, b], 2, 3);
    ///
    /// assert_eq!(image.get_width(), 2);
    /// ```
    fn get_width(&self) -> usize {
        self.width
    }
}

#[cfg(test)]
mod source_2d_test {
    use super::*;
    use crate::pixel::RgbaPixel;

    #[test]
    #[should_panic]
    fn dim_test() {
        Image2D::<RgbaPixel, Vec<RgbaPixel>>::new(vec![], 0, 0);
    }

    #[test]
    #[should_panic]
    fn wrong_dim() {
        let bl = RgbaPixel::from((0, 0, 0, 255));
        Image2D::new(vec![bl, bl, bl], 10, 10);
    }

    #[test]
    fn flatten_coord() {
        let bl = RgbaPixel::from((0, 0, 0, 255));
        assert_eq!(Image2D::new(vec![bl; 12], 3, 4).flatten_coord(2, 2), 8);
    }

    #[test]
    fn expand_coord() {
        let bl = RgbaPixel::from((0, 0, 0, 255));
        assert_eq!(Image2D::new(vec![bl; 12], 3, 4).expand_coord(8), (2, 2));
    }
}
