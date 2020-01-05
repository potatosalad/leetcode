/**
 * [40] Combination Sum II
 *
 * Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * Each number in candidates may only be used once in the combination.
 *
 * Note:
 *
 *
 * 	All numbers (including target) will be positive integers.
 * 	The solution set must not contain duplicate combinations.
 *
 *
 * Example 1:
 *
 *
 * Input: candidates = [10,1,2,7,6,1,5], target = 8,
 * A solution set is:
 * [
 *   [1, 7],
 *   [1, 2, 5],
 *   [2, 6],
 *   [1, 1, 6]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input: candidates = [2,5,2,1,2], target = 5,
 * A solution set is:
 * [
 *   [1,2,2],
 *   [5]
 * ]
 *
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::BTreeSet;
use std::collections::HashSet;

fn find_all_combination_sum2(
    candidates: &[i32],
    target: i32,
    offset: usize,
    solution: Vec<i32>,
    indices: HashSet<usize>,
    solutions: &mut BTreeSet<Vec<i32>>,
) {
    for (i, &candidate) in candidates
        .iter()
        .enumerate()
        .filter(|(_, &candidate)| target - candidate >= 0)
    {
        let mut solution = solution.clone();
        let mut indices = indices.clone();
        if !indices.contains(&(i + offset)) {
            solution.push(candidate);
            indices.insert(i + offset);
            if target == candidate {
                solutions.insert(solution);
                continue;
            }
        }
        find_all_combination_sum2(
            &candidates[(i + 1)..],
            target - candidate,
            i + 1 + offset,
            solution,
            indices,
            solutions,
        );
    }
}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable_by(|a, b| b.cmp(a));
        let mut solutions = BTreeSet::new();
        find_all_combination_sum2(
            &candidates,
            target,
            0,
            Vec::new(),
            HashSet::new(),
            &mut solutions,
        );
        solutions.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0040() {
        assert_eq!(
            vec![vec![1, 1, 1, 1, 1, 1, 1]],
            Solution::combination_sum2(vec![1, 1, 1, 1, 1, 1, 1], 7)
        );
        assert_eq!(
            vec![vec![5, 2, 1], vec![6, 1, 1], vec![6, 2], vec![7, 1]],
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
        );
        assert_eq!(
            vec![vec![2, 2, 1], vec![5]],
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5)
        );
    }
}
