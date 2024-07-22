fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k: i32 = 0  ;
    let mut i: usize = 0;
    let mut end: usize = nums.len();
    while i < end {
        if nums[i] == val {
            // Send the element to the end of the vector
            let tmp = nums[i];
            nums.remove(i);
            nums.push(tmp);
            end -= 1;
        } else {
            k += 1;
            i += 1;
        }
    }

    k
}

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::remove_element;

    fn test(mut nums: Vec<i32>, expected: Vec<i32>, val: i32) {
        let k = remove_element(&mut nums, val);
        assert_eq!(expected.len(), k as usize);
        nums[0..(k as usize)].sort();
        for i in 0..expected.len() {
            assert_eq!(expected[i], nums[i]);
        }
    }

    #[test]
    fn example_1() {
        let nums = vec![3, 2, 2, 3];
        let expected = vec![2, 2];
        let val = 3;
        test(nums, expected, val);
    }

    #[test]
    fn example_2() {
        let nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let expected = vec![0, 0, 1, 3, 4];
        let val = 2;
        test(nums, expected, val);
    }
}
