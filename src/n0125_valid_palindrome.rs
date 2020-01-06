/**
 * [0125] Valid Palindrome
 *
 * Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
 *
 * Note: For the purpose of this problem, we define empty string as valid palindrome.
 *
 * Example 1:
 *
 *
 * Input: "A man, a plan, a canal: Panama"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: "race a car"
 * Output: false
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s.chars().filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        });
        chars.clone().eq(chars.rev())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0125() {
        let test_vectors = vec![
            (true, "A man, a plan, a canal: Panama"),
            (false, "race a car"),
            (true, "race car"),
            (false, "0P"),
        ];
        for (i, &(valid, s)) in test_vectors.iter().enumerate() {
            assert!(
                valid == Solution::is_palindrome(s.to_string()),
                "expected test vector i={:?}, s={:?} to be {}",
                i,
                s,
                valid
            );
        }
    }
}
