use super::Pixel;

/// A pixel with an RGB color representation.
pub struct RgbPixel {
    r: u8,
    g: u8,
    b: u8,
}

impl RgbPixel {
    /// Creates a new [`RgbPixel`]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RgbPixel { r, g, b }
    }
}

impl Pixel for RgbPixel {
    fn blend(self, other: Self, opacity: f32) -> Self {
        // Blend rbg pixels together
        self
    }
}
