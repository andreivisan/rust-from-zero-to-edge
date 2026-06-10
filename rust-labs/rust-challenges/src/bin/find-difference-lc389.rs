/*
 * The idea is to create one array of bytes and use fold to iterate and accumulate
 * the result in acc - fold applies the lambda function to the byte(s).
 * */
pub fn find_the_difference(s: String, t: String) -> char {
    s.bytes().chain(t.bytes()).fold(0u8, |acc, byte| acc ^ byte) as char
}

pub fn main() {}
