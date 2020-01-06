/**
 * [0049] Group Anagrams
 *
 * Given an array of strings, group anagrams together.
 *
 * Example:
 *
 *
 * Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
 * Output:
 * [
 *   ["ate","eat","tea"],
 *   ["nat","tan"],
 *   ["bat"]
 * ]
 *
 * Note:
 *
 *
 * 	All inputs will be in lowercase.
 * 	The order of your output does not matter.
 *
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;

fn hash_anagram(s: &str) -> [u32; 26] {
    let mut key: [u32; 26] = [0; 26];
    for i in s
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase() as u32 - 'a' as u32)
    {
        key[i as usize] += 1;
    }
    key
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u32; 26], Vec<String>> = HashMap::new();
        for s in strs {
            map.entry(hash_anagram(s.as_str()))
                .or_insert_with(Vec::new)
                .push(s);
        }
        let mut groups: Vec<Vec<String>> = map
            .into_iter()
            .map(|(_, mut group)| {
                group.sort_unstable();
                group
            })
            .collect();
        groups.sort_unstable();
        groups
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0049() {
        assert_eq!(
            vec![
                vec_string!["ate", "eat", "tea"],
                vec_string!["bat"],
                vec_string!["nat", "tan"],
            ],
            Solution::group_anagrams(vec_string!["eat", "tea", "tan", "ate", "nat", "bat"])
        );
        assert_eq!(
            vec![vec_string!["", ""]],
            Solution::group_anagrams(vec_string!["", ""])
        );
    }
}
