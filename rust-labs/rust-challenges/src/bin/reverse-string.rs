use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    if input.len() < 2 {
        return input.to_string();
    }
    input.graphemes(true).rev().fold(
        String::with_capacity(input.len()), 
        |mut reversed, grapheme| {
            reversed.push_str(grapheme);
            reversed
        }
    )
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
