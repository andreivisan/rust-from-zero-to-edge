/// Determines whether `input` is a palindrome after ignoring non-alphanumeric
/// ASCII characters and letter casing.
///
/// The input contract is printable ASCII. Aim for `O(n)` time and `O(1)`
/// additional space without constructing a normalized `String`.
pub fn is_valid_palindrome(input: &str) -> bool {
    if input.len() < 2 { return true; }
    let input = input.as_bytes();
    let mut start = 0;
    let mut end = input.len() - 1;
    while start < end {
        //this only woks because the characters are ASCII
        if !(input[start] as char).is_ascii_alphanumeric() { start += 1; continue; }
        if !(input[end] as char).is_ascii_alphanumeric() { end -= 1; continue; }
        if !(input[start] as char).eq_ignore_ascii_case(&(input[end] as char)) { return false; }
        start += 1;
        end -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_valid_palindrome;

    #[test]
    fn accepts_a_phrase_with_spaces_and_punctuation() {
        assert!(is_valid_palindrome("A man, a plan, a canal: Panama"));
    }

    #[test]
    fn rejects_a_non_palindromic_phrase() {
        assert!(!is_valid_palindrome("race a car"));
    }

    #[test]
    fn compares_ascii_letters_without_case() {
        assert!(is_valid_palindrome("No 'x' in Nixon"));
    }

    #[test]
    fn handles_digits_as_significant_characters() {
        assert!(is_valid_palindrome("12321"));
        assert!(!is_valid_palindrome("12a31"));
    }

    #[test]
    fn distinguishes_letters_from_digits() {
        assert!(!is_valid_palindrome("0P"));
    }

    #[test]
    fn accepts_an_empty_string() {
        assert!(is_valid_palindrome(""));
    }

    #[test]
    fn accepts_input_with_only_ignored_characters() {
        assert!(is_valid_palindrome(" ,.!?"));
    }

    #[test]
    fn accepts_one_significant_character() {
        assert!(is_valid_palindrome("...Z..."));
    }
}
