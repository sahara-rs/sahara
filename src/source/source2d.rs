use crate::pixel;

/// A type that can be used as a 2D source
///
/// Types implementing `Source2D` are able to be composited
/// onto other sources via a [`composite`](crate::composite).
pub trait Source2D<T>
where
    T: pixel::Pixel + Default,
{
    /// Lookup a [`Pixel`](pixel::Pixel) from the [`Source2D`](Source2D) with coordinates `x` and `y`.
    fn get_pixel(&self, x: usize, y: usize) -> T;
}
