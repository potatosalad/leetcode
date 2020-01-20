/**
 * [0567] Permutation in String
 *
 * Given two strings s1 and s2, write a function to return true if s2 contains the permutation of s1. In other words, one of the first string's permutations is the substring of the second string.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: s1 = "ab" s2 = "eidbaooo"
 * Output: True
 * Explanation: s2 contains one permutation of s1 ("ba").
 *
 *
 * Example 2:
 *
 *
 * Input:s1= "ab" s2 = "eidboaoo"
 * Output: False
 *
 *
 *  
 *
 * Note:
 *
 *
 * 	The input strings only contain lower case letters.
 * 	The length of both given strings is in range [1, 10,000].
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let len1 = s1.len();
        let len2 = s2.len();
        if len1 > len2 {
            return false;
        }
        let mut count: Vec<i32> = vec![0; 26];
        for i in 0..len1 {
            count[(s1[i] - 'a' as u8) as usize] += 1;
            count[(s2[i] - 'a' as u8) as usize] -= 1;
        }
        if count.iter().all(|&c| c == 0) {
            return true;
        }
        for i in len1..len2 {
            count[(s2[i] - 'a' as u8) as usize] -= 1;
            count[(s2[i - len1] - 'a' as u8) as usize] += 1;
            if count.iter().all(|&c| c == 0) {
                return true;
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
    fn test_0567() {
        assert!(Solution::check_inclusion(
            "ab".to_string(),
            "eidbaooo".to_string()
        ));
        assert!(!Solution::check_inclusion(
            "ab".to_string(),
            "eidboaoo".to_string()
        ));
    }
}
