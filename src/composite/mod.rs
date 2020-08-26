//! A composite is the output of a [`compositor`](crate::compositor).
//!
//! A composite also implements the [`composable`](crate::composable) trait,
//! meaning you can nest composites.
mod composite_2d;

pub use composite_2d::Composite2D;
