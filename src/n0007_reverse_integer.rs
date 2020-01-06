/**
 * [0007] Reverse Integer
 *
 * Given a 32-bit signed integer, reverse digits of an integer.
 *
 * Example 1:
 *
 *
 * Input: 123
 * Output: 321
 *
 *
 * Example 2:
 *
 *
 * Input: -123
 * Output: -321
 *
 *
 * Example 3:
 *
 *
 * Input: 120
 * Output: 21
 *
 *
 * Note:
 * Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut r = 0_i32;
        let mut n = x;
        while n != 0 {
            if let Some(r10) = r.checked_mul(10) {
                r = r10 + n % 10;
                n /= 10;
            } else {
                return 0;
            }
        }
        r
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-123000), -321);
        let base: i64 = 2;
        assert_eq!(Solution::reverse((base.pow(31) - 1) as i32), 0);
        assert_eq!(Solution::reverse((-base.pow(31)) as i32), 0);
    }
}
