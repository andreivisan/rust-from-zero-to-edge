pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut write = 0;
    for read in 0..nums.len() {
        if nums[read] != 0 {
            nums[write] = nums[read];
            write += 1
        }
    }
    nums[write..].fill(0);
}

fn main() {}
