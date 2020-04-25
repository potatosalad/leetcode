/**
 * [0717] 1-bit and 2-bit Characters
 *
 * We have two special characters. The first character can be represented by one bit 0. The second character can be represented by two bits (10 or 11).  
 *
 * Now given a string represented by several bits. Return whether the last character must be a one-bit character or not. The given string will always end with a zero.
 *
 * Example 1:
 *
 * Input:
 * bits = [1, 0, 0]
 * Output: True
 * Explanation:
 * The only way to decode it is two-bit character and one-bit character. So the last character is one-bit character.
 *
 *
 *
 * Example 2:
 *
 * Input:
 * bits = [1, 1, 1, 0]
 * Output: False
 * Explanation:
 * The only way to decode it is two-bit character and two-bit character. So the last character is NOT one-bit character.
 *
 *
 *
 * Note:
 * 1 <= len(bits) <= 1000.
 * bits[i] is always 0 or 1.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() {
            if i == bits.len() - 1 {
                return true;
            } else if bits[i] == 1 {
                i += 2;
            } else {
                i += 1;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0717() {
        assert_eq!(true, Solution::is_one_bit_character(vec![1, 0, 0]));
        assert_eq!(false, Solution::is_one_bit_character(vec![1, 1, 1, 0]));
    }
}
