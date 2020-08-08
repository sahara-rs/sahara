/// A type that can be treated as a pixel.
///
/// Types implementing `Pixel` are able to be used
/// as a `Pixel` associated type in a [`Source2D`](crate::source::Source2D)
pub trait Pixel: Sized {
    /// Blend two pixels at a specified opacity
    fn blend(self, other: Self, opacity: f32) -> Self;
}
