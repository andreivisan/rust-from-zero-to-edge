use std::cmp;

pub fn merge_alternately(word1: String, word2: String) -> String {
    if word1.is_empty() { return word2; }
    if word2.is_empty() { return word1; }
    let min_len = cmp::min(word1.len(), word2.len());
    let word1_tail = &word1[min_len..];
    let word2_tail = &word2[min_len..];
    word1.chars().zip(word2.chars())
        .flat_map(|(a, b)| [a, b])
        .chain(word1_tail)
        .chain(word2_tail)
}
fn main() {}
