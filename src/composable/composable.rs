use crate::pixel;

/// A type that is composable.
///
/// Types implementing `Composable2D` are able to be composited
/// onto other `Composable2D` objects via a [`Composite2D`](crate::composite::Composite2D).
///
/// Types implementing `Composable` should generally be immutable. See [`compositor`](crate::compositor)'s if you want to composite sources.

pub trait Composable2D<T>
where
    T: pixel::Pixel + Default,
{
    /// Lookup a [`Pixel`](pixel::Pixel) with coordinates `x` and `y`.
    fn get_pixel(&self, x: usize, y: usize) -> T;
}
