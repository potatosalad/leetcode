/**
 * [14] Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 * Example 1:
 *
 *
 * Input: ["flower","flow","flight"]
 * Output: "fl"
 *
 *
 * Example 2:
 *
 *
 * Input: ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *
 * Note:
 *
 * All given inputs are in lowercase letters a-z.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let len = strs.iter().map(|s| s.len()).min().unwrap();
        let fst = strs.first().unwrap();
        let mut i = 0;
        while i < len {
            if strs.iter().all(|s| s.as_bytes()[i] == fst.as_bytes()[i]) {
                i += 1;
            } else {
                break;
            }
        }
        (&fst[0..i]).to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0014() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
    }
}
