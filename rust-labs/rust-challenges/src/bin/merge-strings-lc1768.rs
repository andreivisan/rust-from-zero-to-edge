pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut a = word1.chars();
    let mut b = word2.chars();
    let mut result = String::with_capacity(word1.len() + word2.len());
    loop {
        match (a.next(), b.next()) {
            (Some(x), Some(y)) => { result.push(x); result.push(y); }
            (Some(x), None) => { result.push(x); result.extend(a); break; }
            (None, Some(y)) => { result.push(y); result.extend(b); break; }
            (None, None) => break,
        }
    }
    result
}

fn main() {
    let result = merge_alternately("Hello".to_string(), ", Andrei aka Cybermaster!".to_string());
    println!("{result}");
}

