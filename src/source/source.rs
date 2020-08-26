use crate::pixel;

/// A type that is composable.
///
/// Types implementing `Source2D` are able to be composited
/// onto other `Source2D` objects via a [`compositor`](crate::compositor).
///
/// Types implementing `Source2D` should generally be immutable. See the [`compositor`](crate::compositor) module if you want to composite sources.
///

pub trait Source2D<T>
where
    T: pixel::Pixel + Default,
{
    /// Lookup a [`Pixel`](pixel::Pixel) at location `(row, col)`.
    /// The pixel at position `row: 0, col: 0` should be at the top left corner.
    ///
    /// # Bounds
    ///
    /// `0 ≤ row < height`
    ///
    /// `0 ≤ col < width`
    fn get_pixel(&self, row: usize, col: usize) -> &T;

    /// Set a [`Pixel`](pixel::Pixel) at location `(row, col)`.
    /// The pixel at position `row: 0, col: 0` should be at the top left corner.
    ///
    /// # Bounds
    /// Same as [`get_pixel`](crate::source::Source2D::get_pixel)
    fn set_pixel(&mut self, row: usize, col: usize, pixel: T);

    /// Get the width of an image in pixels
    fn get_width(&self) -> usize;

    /// Get the height of an image in pixels
    fn get_height(&self) -> usize;
}
