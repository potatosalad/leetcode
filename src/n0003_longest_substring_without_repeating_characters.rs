/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * Example 1:
 *
 * Input: "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 * Example 2:
 *
 * Input: "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 * Example 3:
 *
 * Input: "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 *              Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashSet;

struct SubstringIterator<'a> {
    s: &'a str,
}

impl<'a> SubstringIterator<'a> {
    pub fn new(s: &'a str) -> Self {
        SubstringIterator { s }
    }
}

impl<'a> Iterator for SubstringIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            None
        } else {
            let mut hs = HashSet::new();
            let mut index = 0;
            for c in self.s.chars() {
                if hs.contains(&c) {
                    break;
                } else {
                    hs.insert(c);
                    index += 1;
                }
            }
            let item = &self.s[0..index];
            self.s = &self.s[1..];
            Some(item)
        }
    }
}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        SubstringIterator::new(s.as_str())
            .map(str::len)
            .max()
            .map_or(0_i32, |l| l as i32)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0003() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
