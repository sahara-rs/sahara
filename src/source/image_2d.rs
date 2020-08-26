use super::Source2D;
use crate::pixel::Pixel;

#[derive(Clone, Debug)]
/// A 2D image.
pub struct Image2D<T>
where 
    T: Pixel
{
    /// Pixels of an object, flattened into a 1D vector
    pixels: Vec<T>,
    /// Width of image in pixels
    width: usize,
    /// Height of image in pixels
    height: usize,
}

impl<T> Image2D<T>
where
    T: Pixel
{
    /// Create a new image from a vector of pixels.
    ///
    /// # Bounds
    ///
    /// `width > 0`
    ///
    /// `height > 0`
    /// 
    /// `width * height == pixels.len()`
    pub fn new<P>(pixels: Vec<P>, width: usize, height: usize) -> Self 
    where
        P: Into<T>,
    {
        assert!(width > 0);
        assert!(height > 0);
        assert_eq!(width * height, pixels.len());

        Self {
            pixels: pixels.into_iter().map(|pix| pix.into()).collect::<Vec<T>>(),
            width,
            height,
        }
    }

    #[inline]
    /// Map 2D coordinates to a 1D coordinate
    fn calc_pos(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

impl<T> Source2D<T> for Image2D<T>
where
    T: Pixel + Default,
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
    /// let mut image_2d: Image2D<RgbaPixel> = Image2D::new(vec![bl; 4], 2, 2);
    /// image_2d.set_pixel(0, 1, r);
    ///
    /// assert_eq!(image_2d.get_pixel(0, 0), &bl); // Top left corner
    /// assert_eq!(image_2d.get_pixel(0, 1), &r);  // Top right corner
    /// assert_eq!(image_2d.get_pixel(1, 0), &bl); // Bottom left corner
    /// assert_eq!(image_2d.get_pixel(1, 1), &bl); // Bottom right corner

    /// ```
    fn set_pixel(&mut self, row: usize, col: usize, pixel: T) {
        let pos_1d = self.calc_pos(row, col);
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
    /// let image_2d: Image2D<RgbaPixel> = Image2D::new(vec![bl, r, g, r], 2, 2); // Create a 2 by 2 image
    ///
    /// assert_eq!(image_2d.get_pixel(0, 0), &bl); // Top left corner
    /// assert_eq!(image_2d.get_pixel(0, 1), &r);  // Top right corner
    /// assert_eq!(image_2d.get_pixel(1, 0), &g);  // Bottom left corner
    /// assert_eq!(image_2d.get_pixel(1, 1), &r);  // Bottom right corner
    /// ```
    fn get_pixel(&self, row: usize, col: usize) -> &T {
        &self.pixels[self.calc_pos(row, col)]
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
    /// let image: Image2D<RgbaPixel> = Image2D::new(vec![b, b, b, b, b, b], 2, 3);
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
    /// let image: Image2D<RgbaPixel> = Image2D::new(vec![b, b, b, b, b, b], 2, 3);
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
        let _: Image2D<RgbaPixel> = Image2D::new::<(u8, u8, u8, u8)>(vec![], 0, 0);
    }

    #[test]
    #[should_panic]
    fn wrong_dim() {
        let bl = RgbaPixel::from((0, 0, 0, 255));
        let _: Image2D<RgbaPixel> = Image2D::new(vec![bl, bl, bl], 10, 10);
    }
}

