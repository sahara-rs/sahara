use super::Pixel;

pub struct RgbPixel {
    r: u8,
    g: u8,
    b: u8,
}

impl RgbPixel {
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
