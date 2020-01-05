/**
 * [41] First Missing Positive
 *
 * Given an unsorted integer array, find the smallest missing positive integer.
 *
 * Example 1:
 *
 *
 * Input: [1,2,0]
 * Output: 3
 *
 *
 * Example 2:
 *
 *
 * Input: [3,4,-1,1]
 * Output: 2
 *
 *
 * Example 3:
 *
 *
 * Input: [7,8,9,11,12]
 * Output: 1
 *
 *
 * Note:
 *
 * Your algorithm should run in O(n) time and uses constant extra space.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut skip = 0;
        let count = nums.len();
        for i in 0..count {
            if nums[i] <= 0 {
                nums.swap(i, skip);
                skip += 1;
            }
        }
        for i in skip..count {
            let idx = skip + (nums[i].abs() - 1) as usize;
            if idx < count && nums[idx] > 0 {
                if let Some(n) = nums.get_mut(idx) {
                    *n = -*n;
                } else {
                    unreachable!();
                }
            }
        }
        for i in 0..(count - skip) {
            if nums[i + skip] > 0 {
                return (i + 1) as i32;
            }
        }
        (count - skip + 1) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_41() {
        assert_eq!(1, Solution::first_missing_positive(vec![2, 2]));
        assert_eq!(
            1,
            Solution::first_missing_positive(vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2])
        );
        assert_eq!(
            1,
            Solution::first_missing_positive(vec![2, 2, 2, 2, 2, 2, 2]),
        );
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
        assert_eq!(3, Solution::first_missing_positive(vec![2, 1, 0]));
        assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
        assert_eq!(
            4,
            Solution::first_missing_positive(vec![7, 8, 1, 2, 3, 3, 3, 3, 3, 3, 3, -5, -7, 1234])
        );
    }
}
