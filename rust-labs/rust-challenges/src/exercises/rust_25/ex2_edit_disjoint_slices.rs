pub fn calibrate(
    readings: &mut [i32],
    split: usize,
    left_offset: i32,
    right_offset: i32,
) -> (i32, i32) {
    if readings.is_empty() {
        return (0, 0);
    }
    let (left, right) = readings.split_at_mut(split);
    apply_offset(left, left_offset);
    apply_offset(right, right_offset);
    (left.iter().sum(), right.iter().sum())
}

fn apply_offset(readings: &mut [i32], offset: i32) {
    for el in readings {
        *el += offset;
    }
}

#[cfg(test)]
mod tests {
    use super::calibrate;

    #[test]
    fn calibrates_both_groups_and_sums_updated_readings() {
        let mut readings = [10, 20, 30, 40, 50];

        let sums = calibrate(&mut readings, 2, 3, -5);

        assert_eq!(readings, [13, 23, 25, 35, 45]);
        assert_eq!(sums, (36, 105));
    }

    #[test]
    fn handles_an_empty_left_group() {
        let mut readings = [-2, 0, 2];

        let sums = calibrate(&mut readings, 0, 100, 3);

        assert_eq!(readings, [1, 3, 5]);
        assert_eq!(sums, (0, 9));
    }

    #[test]
    fn handles_an_empty_right_group() {
        let mut readings = [5, 10, 15];

        let sums = calibrate(&mut readings, 3, -2, 99);

        assert_eq!(readings, [3, 8, 13]);
        assert_eq!(sums, (24, 0));
    }

    #[test]
    fn handles_no_readings() {
        let mut readings = [];

        let sums = calibrate(&mut readings, 0, 5, -5);

        assert_eq!(readings, []);
        assert_eq!(sums, (0, 0));
    }

    #[test]
    fn zero_offsets_preserve_values_and_still_compute_sums() {
        let mut readings = [4, -4, 7];

        let sums = calibrate(&mut readings, 2, 0, 0);

        assert_eq!(readings, [4, -4, 7]);
        assert_eq!(sums, (0, 7));
    }

    #[test]
    fn a_single_reading_belongs_to_exactly_one_group() {
        let mut right_only = [7];
        let right_sums = calibrate(&mut right_only, 0, 100, -2);

        assert_eq!(right_only, [5]);
        assert_eq!(right_sums, (0, 5));

        let mut left_only = [7];
        let left_sums = calibrate(&mut left_only, 1, 3, -100);

        assert_eq!(left_only, [10]);
        assert_eq!(left_sums, (10, 0));
    }

    #[test]
    fn calibrating_a_subslice_preserves_surrounding_readings() {
        let mut readings = [900, 10, 20, 30, 40, 800];

        let sums = calibrate(&mut readings[1..5], 2, 1, -2);

        assert_eq!(readings, [900, 11, 21, 28, 38, 800]);
        assert_eq!(sums, (32, 66));
    }

    #[test]
    fn handles_the_largest_allowed_positive_sum() {
        let mut readings = [10_000; 1_000];

        let sums = calibrate(&mut readings, 1_000, 10_000, -10_000);

        assert_eq!(readings, [20_000; 1_000]);
        assert_eq!(sums, (20_000_000, 0));
    }

    #[test]
    fn handles_the_largest_allowed_negative_sum() {
        let mut readings = [-10_000; 1_000];

        let sums = calibrate(&mut readings, 0, 10_000, -10_000);

        assert_eq!(readings, [-20_000; 1_000]);
        assert_eq!(sums, (0, -20_000_000));
    }

    #[test]
    fn repeated_calls_use_the_updated_values_and_new_split() {
        let mut readings = [1, 2, 3, 4];

        let first_sums = calibrate(&mut readings, 2, 2, -1);
        assert_eq!(readings, [3, 4, 2, 3]);
        assert_eq!(first_sums, (7, 5));

        let second_sums = calibrate(&mut readings, 1, -3, 4);
        assert_eq!(readings, [0, 8, 6, 7]);
        assert_eq!(second_sums, (0, 21));
    }
}
