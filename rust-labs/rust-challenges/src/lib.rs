extern crate self as rust_challenges;

#[allow(dead_code)]
#[path = "bin/reverse-string.rs"]
mod reverse_string;

pub use reverse_string::reverse;
