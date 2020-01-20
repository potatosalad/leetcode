/**
 * [1155] Number of Dice Rolls With Target Sum
 *
 * You have d dice, and each die has f faces numbered 1, 2, ..., f.
 *
 * Return the number of possible ways (out of f^d total ways) modulo 10^9 + 7 to roll the dice so the sum of the face up numbers equals target.
 *
 *  
 * Example 1:
 *
 *
 * Input: d = 1, f = 6, target = 3
 * Output: 1
 * Explanation:
 * You throw one die with 6 faces.  There is only one way to get a sum of 3.
 *
 *
 * Example 2:
 *
 *
 * Input: d = 2, f = 6, target = 7
 * Output: 6
 * Explanation:
 * You throw two dice, each with 6 faces.  There are 6 ways to get a sum of 7:
 * 1+6, 2+5, 3+4, 4+3, 5+2, 6+1.
 *
 *
 * Example 3:
 *
 *
 * Input: d = 2, f = 5, target = 10
 * Output: 1
 * Explanation:
 * You throw two dice, each with 5 faces.  There is only one way to get a sum of 10: 5+5.
 *
 *
 * Example 4:
 *
 *
 * Input: d = 1, f = 2, target = 3
 * Output: 0
 * Explanation:
 * You throw one die with 2 faces.  There is no way to get a sum of 3.
 *
 *
 * Example 5:
 *
 *
 * Input: d = 30, f = 30, target = 500
 * Output: 222616187
 * Explanation:
 * The answer must be returned modulo 10^9 + 7.
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= d, f <= 30
 * 	1 <= target <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
        const MOD: u64 = 1e9 as u64 + 7;
        let mut dp = vec![vec![0; target as usize + 1]; d as usize + 1];
        dp[0][0] = 1;
        for i in 1..=d as usize {
            for j in 1..=f as usize {
                for k in j..=target as usize {
                    dp[i][k] = (dp[i][k] + dp[i - 1][k - j]) % MOD;
                }
            }
        }
        dp[d as usize][target as usize] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1155() {
        assert_eq!(1, Solution::num_rolls_to_target(1, 6, 3));
        assert_eq!(6, Solution::num_rolls_to_target(2, 6, 7));
        assert_eq!(1, Solution::num_rolls_to_target(2, 5, 10));
        assert_eq!(0, Solution::num_rolls_to_target(1, 2, 3));
        assert_eq!(222_616_187, Solution::num_rolls_to_target(30, 30, 500));
    }
}
