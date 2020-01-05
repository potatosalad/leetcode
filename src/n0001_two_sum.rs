/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they
 * add up to a specific target.
 *
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 *
 * Example:
 *
 *
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::BTreeMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut needles = BTreeMap::new();
        let ns = nums.iter().enumerate();
        for (i, n) in ns.clone() {
            let needle = target - *n;
            if let Some(j) = needles.get(&needle).cloned() {
                let mut result = vec![i as i32, j as i32];
                result.sort_unstable();
                return result;
            }
            needles.insert(*n, i);
        }
        unreachable!();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0001() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
