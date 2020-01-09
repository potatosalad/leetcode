/**
 * [0459] Repeated Substring Pattern
 *
 * Given a non-empty string check if it can be constructed by taking a substring of it and appending multiple copies of the substring together. You may assume the given string consists of lowercase English letters only and its length will not exceed 10000.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: "abab"
 * Output: True
 * Explanation: It's the substring "ab" twice.
 *
 *
 * Example 2:
 *
 *
 * Input: "aba"
 * Output: False
 *
 *
 * Example 3:
 *
 *
 * Input: "abcabcabcabc"
 * Output: True
 * Explanation: It's the substring "abc" four times. (And the substring "abcabc" twice.)
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        match s.len() {
            0 => false,
            1 => false,
            _ => {
                let bytes = s.as_bytes();
                let first_byte = bytes[0];
                let mut chunk_size = 1;
                while chunk_size <= bytes.len() / 2 {
                    if bytes.len() % chunk_size != 0 {
                        chunk_size += 1;
                    } else if bytes[chunk_size] == first_byte {
                        let expected = &bytes[0..chunk_size];
                        if bytes
                            .chunks_exact(chunk_size)
                            .all(|chunk| chunk == expected)
                        {
                            return true;
                        } else {
                            chunk_size += 1;
                        }
                    } else {
                        chunk_size += 1;
                    }
                }
                false
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0459() {
        assert!(Solution::repeated_substring_pattern("abab".to_string()));
        assert!(!Solution::repeated_substring_pattern("aba".to_string()));
        assert!(Solution::repeated_substring_pattern(
            "abcabcabcabc".to_string()
        ));
    }
}
