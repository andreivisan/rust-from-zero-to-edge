pub fn repeated_substring_pattern(s: String) -> bool {
    let sb = s.as_bytes();
    let n = sb.len();
    for d in 1..=n / 2 {
        if n % d != 0 { continue; }
        if sb.chunks(d).all(|chunk| chunk == &sb[..d]) { return true; }
    }
    false
} 

fn main() {}

#[cfg(test)]
mod tests {
    use super::repeated_substring_pattern;

    // small helper so the test bodies aren't littered with .to_string()
    fn s(x: &str) -> String {
        x.to_string()
    }

    // "ab" repeated twice
    #[test]
    fn simple_two_char_pattern() {
        assert!(repeated_substring_pattern(s("abab")));
    }

    // not a repetition of any proper substring
    #[test]
    fn not_a_pattern() {
        assert!(!repeated_substring_pattern(s("aba")));
    }

    // "abc" repeated four times
    #[test]
    fn longer_pattern_many_repeats() {
        assert!(repeated_substring_pattern(s("abcabcabcabc")));
    }

    // a single char cannot be built from a *proper* substring of itself
    #[test]
    fn single_char_is_false() {
        assert!(!repeated_substring_pattern(s("a")));
    }

    // "a" repeated twice
    #[test]
    fn two_identical_chars() {
        assert!(repeated_substring_pattern(s("aa")));
    }

    // "a" repeated three times (odd total length, still a valid pattern)
    #[test]
    fn three_identical_chars() {
        assert!(repeated_substring_pattern(s("aaa")));
    }

    // "ab" repeated three times
    #[test]
    fn three_repeats() {
        assert!(repeated_substring_pattern(s("ababab")));
    }

    // all distinct chars, no repetition
    #[test]
    fn all_distinct() {
        assert!(!repeated_substring_pattern(s("abc")));
    }

    // the classic tricky one: period 5 ("abaab" * 2), even though the
    // pattern's first char also reappears *inside* the period.
    #[test]
    fn overlapping_prefix_pattern() {
        assert!(repeated_substring_pattern(s("abaababaab")));
    }

    // looks close to a pattern but isn't ("ab" then "ac")
    #[test]
    fn near_miss() {
        assert!(!repeated_substring_pattern(s("abac")));
    }
}
