/// A type that can be treated as a pixel.
///
/// Types implementing `Pixel` are able to be used
/// as a pixel in [`source`](crate::source)s and [`surface`](crate::surface)s.
pub trait Pixel: Sized {
    /// Blend two pixels at a specified opacity
    ///
    /// If `opacity` is `0.0`, the `self` parameter should be outputted.
    ///
    /// If `opacity` is `1.0`, the `other` parameter should be outputted.
    ///
    /// Anything in between should blend between the two.
    fn blend(&self, other: &Self, opacity: f32) -> Self;
}
