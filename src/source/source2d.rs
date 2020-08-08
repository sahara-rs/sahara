use crate::pixel;

pub trait Source2D<T>
where
    T: pixel::Pixel + Default,
{
    fn get_pixel(&self, x: usize, y: usize) -> T;
}
