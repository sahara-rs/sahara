//! A composite for sources. Used for image compositing.
//! 
//! Composites are mutable, unlike [`sources`](crate::source).
//! Think of a composite as a canvas: you can keep painting on it from sources until it is "done".
//! Once a composite is "done", you can turn it back into a source!
//! This allows for very intuitive layering and grouping.



