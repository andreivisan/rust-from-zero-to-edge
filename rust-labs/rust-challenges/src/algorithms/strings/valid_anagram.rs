pub fn is_anagram(s: String, t: String) -> bool {
    let sb = s.bytes();
    let tb = t.bytes();
    if sb.len() != tb.len() {
        return false;
    }
    let mut count = [0i32; 26];
    for (sc, tc) in sb.zip(tb) {
        count[(sc - b'a') as usize] += 1;
        count[(tc - b'a') as usize] -= 1;
    }
    count.iter().all(|&c| c == 0)
}
