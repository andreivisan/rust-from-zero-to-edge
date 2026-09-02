use std::collections::HashSet;

/// Returns `true` when at least one value occurs more than once.
///
/// The input must remain unchanged. Aim for expected `O(n)` time and
/// `O(n)` additional space.
pub fn contains_duplicate(nums: &[i32]) -> bool {
    let mut uniq = HashSet::new();
    for num in nums {
        if !uniq.insert(num) { return true; }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::contains_duplicate;

    #[test]
    fn detects_adjacent_duplicates() {
        assert!(contains_duplicate(&[1, 2, 2, 3]));
    }

    #[test]
    fn detects_duplicates_far_apart() {
        assert!(contains_duplicate(&[1, 2, 3, 4, 1]));
    }

    #[test]
    fn detects_duplicate_negative_values() {
        assert!(contains_duplicate(&[-7, 2, 4, -7]));
    }

    #[test]
    fn reports_false_when_all_values_are_unique() {
        assert!(!contains_duplicate(&[1, 2, 3, 4]));
    }

    #[test]
    fn reports_false_for_an_empty_slice() {
        assert!(!contains_duplicate(&[]));
    }

    #[test]
    fn reports_false_for_one_element() {
        assert!(!contains_duplicate(&[42]));
    }

    #[test]
    fn handles_repeated_zeroes() {
        assert!(contains_duplicate(&[0, 0]));
    }
}
