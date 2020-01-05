/**
 * [15] 3Sum
 *
 * Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.
 *
 * Note:
 *
 * The solution set must not contain duplicate triplets.
 *
 * Example:
 *
 *
 * Given array nums = [-1, 0, 1, 2, -1, -4],
 *
 * A solution set is:
 * [
 *   [-1, 0, 1],
 *   [-1, -1, 2]
 * ]
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::BTreeSet;

fn two_sum(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
    let mut complements = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for &n in nums {
        let complement = target - n;
        if complements.contains(&complement) {
            let pair = if n < complement {
                (n, complement)
            } else {
                (complement, n)
            };
            if !seen.contains(&pair) {
                out.push(pair);
                seen.insert(pair);
            }
        }
        complements.insert(n);
    }
    out
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let size = nums.len();
        if size < 3 {
            return Vec::new();
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 0;
        let mut out = Vec::new();
        let mut previous = nums[0] - 1;
        while i < size - 2 {
            if nums[i] == previous {
                i += 1;
                continue;
            }
            previous = nums[i];
            for pair in two_sum(&nums[(i + 1)..size], 0 - nums[i]) {
                out.push(vec![nums[i], pair.0, pair.1]);
            }
            i += 1;
        }
        out.sort_unstable();
        out
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum(vec![
                -7, -4, -6, 6, 4, -6, -9, -10, -7, 5, 3, -1, -5, 8, -1, -2, -8, -1, 5, -3, -5, 4,
                2, -5, -4, 4, 7
            ]),
            vec![
                vec![-10, 2, 8],
                vec![-10, 3, 7],
                vec![-10, 4, 6],
                vec![-10, 5, 5],
                vec![-9, 2, 7],
                vec![-9, 3, 6],
                vec![-9, 4, 5],
                vec![-8, 2, 6],
                vec![-8, 3, 5],
                vec![-8, 4, 4],
                vec![-7, -1, 8],
                vec![-7, 2, 5],
                vec![-7, 3, 4],
                vec![-6, -2, 8],
                vec![-6, -1, 7],
                vec![-6, 2, 4],
                vec![-5, -3, 8],
                vec![-5, -2, 7],
                vec![-5, -1, 6],
                vec![-5, 2, 3],
                vec![-4, -4, 8],
                vec![-4, -3, 7],
                vec![-4, -2, 6],
                vec![-4, -1, 5],
                vec![-3, -2, 5],
                vec![-3, -1, 4],
                vec![-2, -1, 3],
                vec![-1, -1, 2]
            ]
        );
        assert_eq!(
            Solution::three_sum(vec![2, 0, -2, -5, -5, -3, 2, -4]),
            vec![vec![-4, 2, 2], vec![-2, 0, 2]]
        );
        let empty_vec: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(vec![]), empty_vec);
    }
}
