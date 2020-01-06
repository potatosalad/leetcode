/**
 * [0009] Palindrome Number
 *
 * Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.
 *
 * Example 1:
 *
 *
 * Input: 121
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
 *
 *
 * Example 3:
 *
 *
 * Input: 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 *
 *
 * Follow up:
 *
 * Coud you solve it without converting the integer to a string?
 *
 */
pub struct Solution {}

// submission codes start here

struct Digits {
    x: Option<u32>,
}

impl Digits {
    pub fn new(x: u32) -> Self {
        Digits { x: Some(x) }
    }
}

impl Iterator for Digits {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut x) = self.x.take() {
            let y = if x == 0 { x } else { x % 10 };
            x /= 10;
            if x == 0 {
                self.x = None;
            } else {
                self.x = Some(x);
            }
            Some(y as u8)
        } else {
            None
        }
    }
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else if x < 10 {
            true
        } else {
            let mut digits: Vec<u8> = Digits::new(x as u32).collect();
            digits.reverse();
            let size = digits.len();
            if digits[0] == 0 {
                false
            } else {
                for i in 0..(size / 2) {
                    if digits[i] != digits[size - 1 - i] {
                        return false;
                    }
                }
                true
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0009() {
        assert_eq!(Solution::is_palindrome(-32), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(9), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(2222), true);
        assert_eq!(Solution::is_palindrome(11222211), true);
    }
}
