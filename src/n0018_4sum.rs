/**
 * [18] 4Sum
 *
 * Given an array nums of n integers and an integer target, are there elements a, b, c, and d in nums such that a + b + c + d = target? Find all unique quadruplets in the array which gives the sum of target.
 *
 * Note:
 *
 * The solution set must not contain duplicate quadruplets.
 *
 * Example:
 *
 *
 * Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.
 *
 * A solution set is:
 * [
 *   [-1,  0, 0, 1],
 *   [-2, -1, 1, 2],
 *   [-2,  0, 0, 2]
 * ]
 *
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter::FromIterator;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.is_empty() || nums.len() < 4 {
            return vec![];
        }
        let twos: BTreeMap<i32, BTreeSet<(i32, i32)>> = nums
            .iter()
            .enumerate()
            .take(nums.len() - 1)
            .flat_map(|(i, &x)| {
                nums.iter()
                    .enumerate()
                    .skip(i + 1)
                    .map(move |(j, &y)| (x + y, (i as i32, j as i32)))
            })
            .fold(BTreeMap::new(), |mut acc, (k, v)| {
                acc.entry(k).or_insert_with(BTreeSet::new).insert(v);
                acc
            });
        let fours: BTreeSet<Vec<i32>> = twos
            .iter()
            .filter_map(|(&sum, ts)| twos.get(&(target - sum)).map(|us| (ts, us)))
            .flat_map(|(ts, us)| ts.iter().flat_map(move |t| us.iter().map(move |u| (t, u))))
            .filter_map(|(&(ti, tj), &(ui, uj))| {
                if ti == ui || ti == uj || tj == ui || tj == uj {
                    None
                } else {
                    let mut vs = vec![
                        nums[ti as usize],
                        nums[tj as usize],
                        nums[ui as usize],
                        nums[uj as usize],
                    ];
                    vs.sort_unstable();
                    Some(vs)
                }
            })
            .collect();
        Vec::from_iter(fours.into_iter())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0018() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }
}
