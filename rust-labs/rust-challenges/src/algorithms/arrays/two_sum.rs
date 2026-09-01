use std::collections::HashMap;

/*
 * Brute force: For each number add with the rest of the elements and if equals
 * target then return i and j.
 *
 * */

pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
     let n = nums.len();
     if n < 2 { return None; }
     let mut dmap: HashMap<i32, usize> = HashMap::new();
     for (i, &num) in nums.iter().enumerate() {
         let diff = target - num;
         match dmap.get(&diff) {
             Some(val) => { return Some((*val, i)); },
             None => { dmap.insert(num, i); },
         };
     }
     None
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn finds_pair_at_the_beginning() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
    }

    #[test]
    fn finds_pair_after_skipping_an_invalid_self_match() {
        assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));
    }

    #[test]
    fn handles_duplicate_values_at_distinct_indices() {
        assert_eq!(two_sum(&[3, 3], 6), Some((0, 1)));
    }

    #[test]
    fn handles_duplicate_zeroes() {
        assert_eq!(two_sum(&[0, 4, 3, 0], 0), Some((0, 3)));
    }

    #[test]
    fn handles_negative_values() {
        assert_eq!(two_sum(&[-3, 4, 3, 90], 0), Some((0, 2)));
    }

    #[test]
    fn returns_none_when_no_pair_exists() {
        assert_eq!(two_sum(&[1, 2, 3], 100), None);
    }

    #[test]
    fn returns_none_for_an_empty_slice() {
        assert_eq!(two_sum(&[], 5), None);
    }

    #[test]
    fn returns_none_for_one_element() {
        assert_eq!(two_sum(&[5], 10), None);
    }
}
