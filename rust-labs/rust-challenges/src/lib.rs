extern crate self as rust_challenges;

#[allow(dead_code)]
#[path = "bin/reverse-string.rs"]
mod reverse_string;
#[allow(dead_code)]
#[path = "bin/gigasecond.rs"]
mod gigasecond;

pub use reverse_string::reverse;
pub use gigasecond::after;
