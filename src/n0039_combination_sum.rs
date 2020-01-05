/**
 * [39] Combination Sum
 *
 * Given a set of candidate numbers (candidates) (without duplicates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * The same repeated number may be chosen from candidates unlimited number of times.
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
 * Input: candidates = [2,3,6,7], target = 7,
 * A solution set is:
 * [
 *   [7],
 *   [2,2,3]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input: candidates = [2,3,5], target = 8,
 * A solution set is:
 * [
 *   [2,2,2,2],
 *   [2,3,3],
 *   [3,5]
 * ]
 *
 *
 */
pub struct Solution {}

// submission codes start here

fn find_all_combination_sum(
    candidates: &[i32],
    target: i32,
    template: Vec<i32>,
    solutions: &mut Vec<Vec<i32>>,
) {
    for (i, &candidate) in candidates
        .iter()
        .enumerate()
        .filter(|(_, &candidate)| target - candidate >= 0)
    {
        let mut solution = template.clone();
        solution.push(candidate);
        if target == candidate {
            solutions.push(solution);
        } else {
            find_all_combination_sum(&candidates[i..], target - candidate, solution, solutions);
        }
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable_by(|a, b| b.cmp(a));
        let mut solutions = Vec::new();
        find_all_combination_sum(&candidates, target, Vec::new(), &mut solutions);
        solutions
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39() {
        assert_eq!(
            Solution::combination_sum(vec![1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![7], vec![3, 2, 2],]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![5, 3], vec![3, 3, 2], vec![2, 2, 2, 2],]
        );
    }
}
