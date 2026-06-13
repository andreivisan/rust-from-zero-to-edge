pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() { return 0; }        
    let h = haystack.as_bytes();
    let n = needle.as_bytes();
    h.windows(n.len()).position(|w| w == n).map_or(-1, |i| i as i32)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::str_str;

    // small helper so the test bodies aren't littered with .to_string()
    fn s(x: &str) -> String {
        x.to_string()
    }

    #[test]
    fn match_in_middle() {
        assert_eq!(str_str(s("hello"), s("ll")), 2);
    }

    #[test]
    fn match_at_start() {
        assert_eq!(str_str(s("sadbutsad"), s("sad")), 0);
    }

    #[test]
    fn no_match() {
        assert_eq!(str_str(s("leetcode"), s("leeto")), -1);
    }

    #[test]
    fn single_char_match() {
        assert_eq!(str_str(s("a"), s("a")), 0);
    }

    #[test]
    fn match_at_end() {
        assert_eq!(str_str(s("abc"), s("c")), 2);
    }

    #[test]
    fn needle_longer_than_haystack() {
        assert_eq!(str_str(s("abc"), s("abcd")), -1);
    }

    // the repeated-first-char case from the lesson: the real match starts
    // at index 4, after an earlier partial match fails.
    #[test]
    fn repeated_first_char() {
        assert_eq!(str_str(s("mississippi"), s("issip")), 4);
    }

    // the overlapping case: windows(2) over "aaa" yields [a,a],[a,a]; the
    // first one already matches, so the answer is 0.
    #[test]
    fn overlapping_first_match_wins() {
        assert_eq!(str_str(s("aaa"), s("aa")), 0);
    }

    // convention check: an empty needle should return 0, not -1.
    #[test]
    fn empty_needle_returns_zero() {
        assert_eq!(str_str(s("abc"), s("")), 0);
    }
}
