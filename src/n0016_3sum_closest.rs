/**
 * [16] 3Sum Closest
 *
 * Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.
 *
 * Example:
 *
 *
 * Given array nums = [-1, 2, 1, -4], and target = 1.
 *
 * The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let size = nums.len();
        if size < 4 {
            return nums.iter().cloned().sum();
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..(size - 1) {
            let mut j = i + 1;
            let mut k = size - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if i32::abs(target - sum) < i32::abs(target - closest) {
                    closest = sum;
                }
                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        closest
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3], 1), 6);
        assert_eq!(
            Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
            82
        );
    }
}
