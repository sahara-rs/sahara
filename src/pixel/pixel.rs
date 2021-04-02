/// A type that can be treated as a pixel.
///
/// Types implementing `Pixel` are able to be used
/// as a pixel in [`source`](crate::source)s and [`surface`](crate::surface)s.
pub trait Pixel: Sized {
    /// Blend two pixels at a specified opacity
    /// The `self` pixel should be under the `other` pixel.
    /// In other words, `other` overlaps `self`, whatever that means for the type of pixel you
    /// implement.
    fn blend(&self, other: &Self) -> Self;
}
