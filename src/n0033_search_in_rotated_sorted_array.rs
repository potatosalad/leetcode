/**
 * [0033] Search in Rotated Sorted Array
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
 *
 * You are given a target value to search. If found in the array return its index, otherwise return -1.
 *
 * You may assume no duplicate exists in the array.
 *
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * Example 1:
 *
 *
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut span = nums.len();
        let mut root = 0;
        while span > 0 {
            let half = span / 2;
            let base = root + half;
            if nums[base] == target {
                return base as i32;
            }
            let lhs = nums[root];
            let rhs = nums[base];
            if !((lhs < rhs && target >= lhs && target <= rhs)
                || (lhs > rhs && target >= lhs && target <= rhs))
            {
                root = base;
            }
            span -= half;
        }
        if nums[root] == target {
            root as i32
        } else {
            -1
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_0033() {
        assert_eq!(Solution::search(vec![7, 8, 1, 2, 3, 4, 5, 6], 2), 3);
        assert_eq!(
            Solution::search(
                vec![
                    1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 0, 1, 2, 3, 4, 5, 6, 7, 8
                ],
                0
            ),
            9
        );
        assert_eq!(
            Solution::search(
                vec![
                    1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 0, 1, 2, 3, 4, 5, 6, 7, 8
                ],
                1006
            ),
            2
        );
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![], 3), -1);
    }
}
