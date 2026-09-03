/// Returns the greatest area formed by two vertical lines.
///
/// A pair at indices `left` and `right` holds
/// `min(heights[left], heights[right]) * (right - left)` units of water.
/// Return `0` when fewer than two lines are provided.
///
/// Aim for `O(n)` time and `O(1)` additional space.
pub fn max_area(_heights: &[u32]) -> u64 {
    todo!("implement max_area")
}

#[cfg(test)]
mod tests {
    use super::max_area;

    #[test]
    fn finds_the_best_non_adjacent_pair() {
        assert_eq!(max_area(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn handles_exactly_two_lines() {
        assert_eq!(max_area(&[1, 1]), 1);
    }

    #[test]
    fn favors_width_when_edge_heights_match() {
        assert_eq!(max_area(&[4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn handles_strictly_decreasing_heights() {
        assert_eq!(max_area(&[5, 4, 3, 2, 1]), 6);
    }

    #[test]
    fn handles_zero_height_lines() {
        assert_eq!(max_area(&[0, 2, 0, 4]), 4);
    }

    #[test]
    fn returns_zero_for_an_empty_slice() {
        assert_eq!(max_area(&[]), 0);
    }

    #[test]
    fn returns_zero_for_one_line() {
        assert_eq!(max_area(&[10]), 0);
    }

    #[test]
    fn returns_area_as_u64() {
        assert_eq!(max_area(&[u32::MAX, u32::MAX]), u32::MAX as u64);
    }
}
