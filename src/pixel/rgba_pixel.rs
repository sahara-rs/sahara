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
        Self { r, g, b, a }
    }
}

impl Pixel for RgbaPixel {
    fn blend(&self, other: &Self) -> Self {
        // Blend rbga pixels together
        let Self {
            r: r_a,
            g: g_a,
            b: b_a,
            a: a_a,
        } = self;
        let Self {
            r: r_b,
            g: g_b,
            b: b_b,
            a: a_b,
        } = other;
        let a = a_a + (a_b * (255 - a_a) / 255);
        let r = (r_a * a_a + r_b * a_b * (255 - a_a) / 255) / a;
        let g = (g_a * a_a + g_b * a_b * (255 - a_a) / 255) / a;
        let b = (b_a * a_a + b_b * a_b * (255 - a_a) / 255) / a;
        Self { r, g, b, a }
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
