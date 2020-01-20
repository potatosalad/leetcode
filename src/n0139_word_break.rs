/**
 * [0139] Word Break
 *
 * Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, determine if s can be segmented into a space-separated sequence of one or more dictionary words.
 *
 * Note:
 *
 *
 * 	The same word in the dictionary may be reused multiple times in the segmentation.
 * 	You may assume the dictionary does not contain duplicate words.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "leetcode", wordDict = ["leet", "code"]
 * Output: true
 * Explanation: Return true because "leetcode" can be segmented as "leet code".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "applepenapple", wordDict = ["apple", "pen"]
 * Output: true
 * Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
 *              Note that you are allowed to reuse a dictionary word.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
 * Output: false
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        fn solution(
            s: &str,
            dict: &[String],
            offset: usize,
            marker: &mut Vec<Option<bool>>,
        ) -> bool {
            if s.is_empty() {
                return true;
            }
            if let Some(matched) = marker[offset] {
                return matched;
            }
            for word in dict {
                if s.starts_with(word)
                    && solution(&s[word.len()..], dict, offset + word.len(), marker)
                {
                    marker[offset] = Some(true);
                    return true;
                }
            }
            marker[offset] = Some(false);
            false
        }
        solution(&s, &word_dict, 0, &mut vec![None; s.len() + 1])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0139() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            vec_string!["leet", "code"]
        ));
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            vec_string!["apple", "pen"]
        ));
        assert!(!Solution::word_break(
            "catsandog".to_string(),
            vec_string!["cats", "dog", "sand", "and", "cat"]
        ));
    }
}
