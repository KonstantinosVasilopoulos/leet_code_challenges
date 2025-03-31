use std::cmp::{max, min};

fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() <= 1 {
        return intervals;
    }

    // Sort the given intervals array using the start of each interval
    intervals.sort_by_key(|interval| interval[0]);

    // Iterate over the intervals vector and merge overlapping intervals
    let mut i: usize = 1;
    let mut intervals_length = intervals.len();
    while i < intervals_length {
        if intervals[i - 1][1] >= intervals[i][0] {
            // Merge and then remove intervals due to overlap
            intervals.insert(
                i,
                vec![
                    min(intervals[i - 1][0], intervals[i][0]),
                    max(intervals[i - 1][1], intervals[i][1]),
                ]
            );
            intervals.remove(i + 1);
            intervals.remove(i - 1);

            // Do not update the i counter, ensuring the new merged interval is compared with its next one
        } else {
            i += 1;
        }

        intervals_length = intervals.len();
    }

    intervals
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::merge;

    // Compares two Vec<Vec<i32>>
    fn eq_interval_vecs(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> bool {
        if a.len() != b.len() {
            return  false;
        }

        for i in 0..a.len() {
            if a[i][0] != b[i][0] || a[i][1] != b[i][1] {
                return false;
            }
        }

        true
    }

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![
            vec![15,18],
            vec![2,6],
            vec![1,3],
            vec![8,10],
        ];
        let result = vec![
            vec![1,6],
            vec![8,10],
            vec![15,18],
        ];
        assert!(eq_interval_vecs(&merge(intervals), &result));
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![
            vec![1,4],
            vec![4,5],
        ];
        let result = vec![
            vec![1,5],
        ];
        assert!(eq_interval_vecs(&merge(intervals), &result));
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![
            vec![1,4],
            vec![5,6],
        ];
        let result = vec![
            vec![1,4],
            vec![5,6],
        ];
        assert!(eq_interval_vecs(&merge(intervals), &result));
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![
            vec![2,3],
            vec![2,2],
            vec![3,3],
            vec![1,3],
            vec![5,7],
            vec![2,2],
            vec![4,6],
        ];
        let result = vec![
            vec![1,3],
            vec![4,7],
        ];
        assert!(eq_interval_vecs(&merge(intervals), &result));
    }
}
