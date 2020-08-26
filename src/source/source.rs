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
    /// Lookup a [`Pixel`](pixel::Pixel) with coordinates `x` and `y`.
    ///
    /// Bounds:
    /// 0 <= `x` < width from `get_width`.
    /// 0 <= `y` < height from `get_height`.
    ///
    /// The pixel at position `x: 0, y: 0` should be at the top left corner.
    fn get_pixel(&self, x: usize, y: usize) -> &T;

    /// Get the width of an image in pixels
    fn get_width(&self) -> usize;

    /// Get the height of an image in pixels
    fn get_height(&self) -> usize;
}
