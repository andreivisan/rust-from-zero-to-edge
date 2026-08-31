pub mod algorithms;
pub mod data_structures;
pub mod exercises;

// Preserve the original crate-level API used by the existing exercises.
pub use algorithms::strings::reverse_string::reverse;
pub use exercises::gigasecond::after;
