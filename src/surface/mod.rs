//! A surface is the output of a [`compositor`](crate::compositor).
//!
//! A surface also implements the [`source`](crate::source) trait,
//! meaning you can nest surfaces.
mod surface_2d;

pub use surface_2d::Surface2D;
