use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let word = input.to_string();
    if word.len() < 2 {
        return word;
    }
    let mut letters = input.graphemes(true).collect::<Vec<&str>>();
    if is_palindrome(&letters) {
        return word;
    }
    let mut start = 0;
    let mut end = letters.len() - 1;
    while start < end {
        letters.swap(start, end);
        start += 1;
        end -= 1;
    }
    println!("{:?}", letters);
    letters.concat()
}

pub fn is_palindrome(letters: &[&str]) -> bool {
    let mut start = 0;
    let mut end = letters.len() - 1;
    while start < end {
        if letters[start] != letters[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

fn main() {
    let input = std::env::args().nth(1).unwrap_or_default();
    println!("{}", reverse(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverses_ascii_text() {
        assert_eq!(reverse("hello"), "olleh");
    }

    #[test]
    fn handles_empty_string() {
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn preserves_unicode_graphemes() {
        assert_eq!(reverse("a\u{0301}b"), "ba\u{0301}");
    }

    #[test]
    fn reverses_emoji_text() {
        assert_eq!(reverse("hi 👋"), "👋 ih");
    }
}
