//! An image processing library.
//!
//! # Terms
//!
//! | Name | Description |
//! | ---- | ----------- |
//! | [`surface`](crate::surface) | A surface is a collection of types implementing [`source`](crate::source). Sources are the result of a [`compositor`](crate::compositor). |
//! | [`source`](crate::source) | A source is a type that is capable of being composed by a [`compositor`](crate::compositor).
//! | [`compositor`](crate::compositor) | A compositor is responsible for combining/compositing types implementing [`source`](crate::source). Compositors output [`surface`](crate::surface)s on command.

#![deny(missing_docs, missing_crate_level_docs)]
#![warn(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_copy_implementations
)]

pub mod compositor;
pub mod pixel;
pub mod source;
pub mod surface;
