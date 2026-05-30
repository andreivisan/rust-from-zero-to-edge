use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let word = input.to_string();
    if word.len() < 2 { return word; }
    let mut letters = input.graphemes(true).collect::<Vec<&str>>();
    if is_palindrome(&letters) { return word; }
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
