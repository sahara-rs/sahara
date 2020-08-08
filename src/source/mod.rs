//! Representation of sources (e.g. a 2D image). Sources are immutable.
//!
//! Note that sources are agnostic to the filetype (e.g. jpeg and png).
//! They can be constructed by hand, i.e. hardcoding the values for each pixel.
//!
//! Sources are **always** immutable. See [`composites`](crate::composite) if you want to composite sources.

mod source_2d;

pub use source_2d::Source2D;
