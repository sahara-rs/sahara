//! An image processing library.
//!
//! ## Terms
//! 
//! | Name                              | Description |
//! |-----------------------------------| ----------- |
//! | [`composite`](crate::composite)   | A composite is a collection of [`composable`](crate::composable) items. Composites are the result of a [`compositor`](crate::compositor). |
//! | [`composable`](crate::composable) | A composable is a type that is capable of being composed by a [`compositor`](crate::compositor).
//! | [`compositor`](crate::compositor) | A compositor is responsible for combining/compositing [`composable`](crate::composable) types. Compositors can output [`composite`](crate::composite)s on command.

#![deny(missing_docs, missing_crate_level_docs)]
#![warn(rust_2018_idioms)]

pub mod composite;
pub mod composable;
pub mod compositor;
pub mod pixel;
pub mod sources;
