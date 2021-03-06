/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
 *
 * Example 1:
 *
 *
 * Input: "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 *
 *
 * Example 2:
 *
 *
 * Input: "cbbd"
 * Output: "bb"
 *
 *
 */
pub struct Solution {}
use unicode_segmentation::UnicodeSegmentation;

// submission codes start here

fn is_palindrome(bytes: &&[u8]) -> bool {
    let half_length = bytes.len() / 2;
    bytes
        .iter()
        .take(half_length)
        .eq(bytes.iter().rev().take(half_length))
}

fn is_palindrome_str(s: &str) -> bool {
    let half_length = s.len() / 2;
    s.chars()
        .take(half_length)
        .eq(s.chars().rev().take(half_length))
}

fn is_palindrome_unicode(graphemes: &&[&str]) -> bool {
    let half_length = graphemes.len() / 2;
    graphemes
        .iter()
        .take(half_length)
        .eq(graphemes.iter().rev().take(half_length))
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        (0..s.len())
            .map(|x| x + 1)
            .rev()
            .flat_map(|x| s.as_bytes().windows(x).filter(is_palindrome))
            .map(|bytes| String::from_utf8(bytes.to_vec()).unwrap())
            .next()
            .unwrap_or_default()
    }

    pub fn longest_palindrome_unicode(s: String) -> String {
        let v: Vec<&str> = s.graphemes(true).collect();
        (0..v.len())
            .map(|x| x + 1)
            .rev()
            .flat_map(|x| v.windows(x).filter(is_palindrome_unicode))
            .map(|graphemes| graphemes.join(""))
            .next()
            .unwrap_or_default()
    }

    pub fn longest_palindrome_str(s: String) -> String {
        let mut out = String::new();
        for (i, c) in s.chars().enumerate() {
            for j in s.rmatch_indices(c).map(|(x, _)| x).filter(|&x| x >= i) {
                let ss = &s[i..=j];
                if is_palindrome_str(ss) && ss.len() > out.len() {
                    out = ss.to_string();
                }
            }
        }
        out
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0005() {
        assert_eq!(Solution::longest_palindrome("aaaaa".to_owned()), "aaaaa");
        assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
        assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("".to_owned()), "");
        assert_eq!(Solution::longest_palindrome_unicode("ay̆ya".to_owned()), "a");
        assert_eq!(
            Solution::longest_palindrome_unicode("ay̆y̆a".to_owned()),
            "ay̆y̆a"
        );
        assert_eq!(
            Solution::longest_palindrome_str("aaaaa".to_owned()),
            "aaaaa"
        );
        assert_eq!(
            Solution::longest_palindrome_str("babab".to_owned()),
            "babab"
        );
        assert_eq!(Solution::longest_palindrome_str("babcd".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome_str("cbbd".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome_str("bb".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome_str("".to_owned()), "");
    }
}
