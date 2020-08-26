use super::Pixel;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// A pixel with an RGB color representation.
pub struct RgbaPixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl RgbaPixel {
    /// Creates a new [`RgbaPixel`]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RgbaPixel { r, g, b, a }
    }
}

impl Pixel for RgbaPixel {
    fn blend(self, other: Self, opacity: f32) -> Self {
        // Blend rbg pixels together
        self
    }
}

impl From<(u8, u8, u8, u8)> for RgbaPixel {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
            a: value.3,
        }
    }
}

