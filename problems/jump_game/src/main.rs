fn can_jump(nums: Vec<i32>) -> bool {
    // The maximum index that can be reached
    let mut max_reachable = 0usize;
    for i in 0..nums.len() - 1 {
        max_reachable = max_reachable.max(i + nums[i] as usize);
        // Check if there was any progress
        if max_reachable == i {
            return false;
        }
    }

    true
}

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::can_jump;

    fn test(nums: Vec<i32>, expected: bool) {
        assert_eq!(expected, can_jump(nums));
    }

    #[test]
    fn example_1() {
        test(vec![2, 3, 1, 1, 4], true);
    }

    #[test]
    fn example_2() {
        test(vec![3, 2, 1, 0, 4], false);
    }
}
