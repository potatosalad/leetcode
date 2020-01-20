/**
 * [0953] Verifying an Alien Dictionary
 *
 * In an alien language, surprisingly they also use english lowercase letters, but possibly in a different order. The order of the alphabet is some permutation of lowercase letters.
 * Given a sequence of words written in the alien language, and the order of the alphabet, return true if and only if the given words are sorted lexicographicaly in this alien language.
 *  
 * Example 1:
 *
 * Input: words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
 * Output: true
 * Explanation: As 'h' comes before 'l' in this language, then the sequence is sorted.
 *
 * Example 2:
 *
 * Input: words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
 * Output: false
 * Explanation: As 'd' comes after 'l' in this language, then words[0] > words[1], hence the sequence is unsorted.
 *
 * Example 3:
 *
 * Input: words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
 * Output: false
 * Explanation: The first three characters "app" match, and the second string is shorter (in size.) According to lexicographical rules "apple" > "app", because 'l' > '&empty;', where '&empty;' is defined as the blank character which is less than any other character (More info</a>).
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 100
 * 	1 <= words[i].length <= 20
 * 	order.length == 26
 * 	All characters in words[i] and order are English lowercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut alphabet: [usize; 26] = [0; 26];
        for (i, c) in order.chars().enumerate() {
            alphabet[(c as u8 - 'a' as u8) as usize] = i;
        }
        let mut sorted_words = words.clone();
        sorted_words.sort_unstable_by_key(|word| {
            word.chars()
                .map(|c| alphabet[(c as u8 - 'a' as u8) as usize])
                .collect::<Vec<usize>>()
        });
        words.eq(&sorted_words)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0953() {
        assert!(Solution::is_alien_sorted(
            vec_string!["hello", "leetcode"],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ));
        assert!(!Solution::is_alien_sorted(
            vec_string!["word", "world", "row"],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ));
        assert!(!Solution::is_alien_sorted(
            vec_string!["apple", "app"],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ));
    }
}
