/**
 * [0140] Word Break II
 *
 * Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, add spaces in s to construct a sentence where each word is a valid dictionary word. Return all such possible sentences.
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
 * Input:
 * s = "catsanddog"
 * wordDict = ["cat", "cats", "and", "sand", "dog"]
 * Output:
 * [
 *   "cats and dog",
 *   "cat sand dog"
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input:
 * s = "pineapplepenapple"
 * wordDict = ["apple", "pen", "applepen", "pine", "pineapple"]
 * Output:
 * [
 *   "pine apple pen apple",
 *   "pineapple pen apple",
 *   "pine applepen apple"
 * ]
 * Explanation: Note that you are allowed to reuse a dictionary word.
 *
 *
 * Example 3:
 *
 *
 * Input:
 * s = "catsandog"
 * wordDict = ["cats", "dog", "sand", "and", "cat"]
 * Output:
 * []
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::BTreeSet;

fn find_word_breaks(
    s: &str,
    dict: &[String],
    offset: usize,
    path: &[&str],
    results: &mut BTreeSet<String>,
    marker: &mut Vec<Option<bool>>,
) -> bool {
    if s.is_empty() {
        results.insert(path.join(" "));
        return true;
    }
    if let Some(matched) = marker[offset] {
        return matched;
    }
    let mut flag: bool = false;
    for word in dict {
        if s.starts_with(word) {
            let mut new_path: Vec<&str> = path.to_vec();
            new_path.push(word.as_str());
            if find_word_breaks(
                &s[word.len()..],
                dict,
                offset + word.len(),
                &new_path,
                results,
                marker,
            ) && !flag
            {
                flag = true;
            }
        }
    }
    if !flag {
        marker[offset] = Some(flag);
        flag
    } else {
        flag
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut results: BTreeSet<String> = BTreeSet::new();
        if find_word_breaks(
            &s,
            &word_dict,
            0,
            &[],
            &mut results,
            &mut vec![None; s.len() + 1],
        ) {
            results.into_iter().collect()
        } else {
            vec![]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0140() {
        assert_eq!(
            vec_string!["cat sand dog", "cats and dog"],
            Solution::word_break(
                "catsanddog".to_string(),
                vec_string!["cat", "cats", "and", "sand", "dog"]
            )
        );
        assert_eq!(
            vec_string![
                "pine apple pen apple",
                "pine applepen apple",
                "pineapple pen apple"
            ],
            Solution::word_break(
                "pineapplepenapple".to_string(),
                vec_string!["apple", "pen", "applepen", "pine", "pineapple"]
            )
        );
        assert_eq!(
            vec![] as Vec<String>,
            Solution::word_break(
                "catsandog".to_string(),
                vec_string!["cats", "dog", "sand", "and", "cat"]
            )
        );
    }
}
