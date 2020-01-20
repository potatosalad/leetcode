/**
 * [0028] Implement strStr()
 *
 * Implement strStr()</a>.
 *
 * Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
 *
 * Example 1:
 *
 *
 * Input: haystack = "hello", needle = "ll"
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: haystack = "aaaaa", needle = "bba"
 * Output: -1
 *
 *
 * Clarification:
 *
 * What should we return when needle is an empty string? This is a great question to ask during an interview.
 *
 * For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr()</a> and Java's indexOf()</a>.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            0
        } else {
            haystack.find(&needle).map_or(-1_i32, |i| i as i32)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0028() {
        assert_eq!(2, Solution::str_str("hello".to_string(), "ll".to_string()));
        assert_eq!(
            -1,
            Solution::str_str("aaaaa".to_string(), "bba".to_string())
        );
    }
}
