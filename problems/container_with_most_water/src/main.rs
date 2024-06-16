use std::cmp::{min, max};

fn max_area(height: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = height.len() - 1;
    let mut maximum_area = 0;
    while left < right {
        let area = (right - left) as i32 * min(height[left], height[right]);
        maximum_area = max(maximum_area, area);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    maximum_area
}


fn main() {
}

#[cfg(test)]
mod test {
    use crate::max_area;

    #[test]
    fn test_full() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(max_area(height), 49);
    }

    #[test]
    fn test_simple() {
        let height = vec![1, 1];
        assert_eq!(max_area(height), 1);
    }
}
