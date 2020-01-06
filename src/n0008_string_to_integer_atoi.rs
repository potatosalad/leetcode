/**
 * [0008] String to Integer (atoi)
 *
 * Implement <span>atoi</span> which converts a string to an integer.
 *
 * The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.
 *
 * The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.
 *
 * If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.
 *
 * If no valid conversion could be performed, a zero value is returned.
 *
 * Note:
 *
 *
 * 	Only the space character ' ' is considered as whitespace character.
 * 	Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. If the numerical value is out of the range of representable values, INT_MAX (2^31 - 1) or INT_MIN (-2^31) is returned.
 *
 *
 * Example 1:
 *
 *
 * Input: "42"
 * Output: 42
 *
 *
 * Example 2:
 *
 *
 * Input: "   -42"
 * Output: -42
 * Explanation: The first non-whitespace character is '-', which is the minus sign.
 *              Then take as many numerical digits as possible, which gets 42.
 *
 *
 * Example 3:
 *
 *
 * Input: "4193 with words"
 * Output: 4193
 * Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
 *
 *
 * Example 4:
 *
 *
 * Input: "words and 987"
 * Output: 0
 * Explanation: The first non-whitespace character is 'w', which is not a numerical
 *              digit or a +/- sign. Therefore no valid conversion could be performed.
 *
 * Example 5:
 *
 *
 * Input: "-91283472332"
 * Output: -2147483648
 * Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
 *              Thefore INT_MIN (-2^31) is returned.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let nstr = str
            .trim()
            .char_indices()
            .take_while(|&(i, c)| (i == 0 && (c == '+' || c == '-')) || c.is_ascii_digit())
            .map(|(_, c)| c)
            .collect::<String>();
        nstr.parse::<i64>().ok().map_or_else(
            || {
                if nstr.len() > 11 {
                    if nstr.chars().nth(0).unwrap() == '-' {
                        i32::min_value()
                    } else {
                        i32::max_value()
                    }
                } else {
                    0
                }
            },
            |v| {
                if v > i32::max_value() as i64 {
                    i32::max_value()
                } else if v < i32::min_value() as i64 {
                    i32::min_value()
                } else {
                    v as i32
                }
            },
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0008() {
        assert_eq!(Solution::my_atoi("aa".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
    }
}
