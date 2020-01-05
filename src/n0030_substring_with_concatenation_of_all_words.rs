/**
 * [30] Substring with Concatenation of All Words
 *
 * You are given a string, s, and a list of words, words, that are all of the same length. Find all starting indices of substring(s) in s that is a concatenation of each word in words exactly once and without any intervening characters.
 *  
 * Example 1:
 *
 * Input:
 *   s = "barfoothefoobarman",
 *   words = ["foo","bar"]
 * Output: [0,9]
 * Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively.
 * The output order does not matter, returning [9,0] is fine too.
 *
 * Example 2:
 *
 * Input:
 *   s = "wordgoodgoodgoodbestword",
 *   words = ["word","good","best","word"]
 * Output: []
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::hash_map::Entry;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct WordSearch {
    need: i32,
    seen: i32,
}

impl WordSearch {
    fn inc_need(&mut self) {
        self.need += 1;
    }

    fn inc_seen(&mut self) {
        self.seen += 1;
    }

    fn dec_seen(&mut self) {
        self.seen -= 1;
    }

    fn has_seen_too_much(&self) -> bool {
        self.seen > self.need
    }

    fn clear(&mut self) {
        self.seen = 0;
    }
}

fn clear_search_state<'a>(state: &mut HashMap<&'a str, WordSearch>) {
    state.iter_mut().for_each(|(_, ws)| ws.clear());
}

fn find_search_state<'a, F>(
    s: &'a str,
    window: usize,
    offset: usize,
    count: usize,
    state: &mut HashMap<&'a str, WordSearch>,
    mut cb: F,
) where
    F: FnMut(i32),
{
    let mut i = 0;
    let mut j = 0;
    while j + window - 1 < s.len() {
        match state.entry(&s[j..(j + window)]) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().inc_seen();
                if entry.get().has_seen_too_much() {
                    while i < j {
                        let word = &s[i..(i + window)];
                        state.entry(word).and_modify(|ws| ws.dec_seen());
                        i += window;
                        if word == &s[j..(j + window)] {
                            break;
                        }
                    }
                    j += window;
                } else if j - i < (count - 1) * window {
                    j += window;
                } else {
                    cb((i + offset) as i32);
                    state
                        .entry(&s[i..(i + window)])
                        .and_modify(|ws| ws.dec_seen());
                    j += window;
                    i += window;
                }
            }
            Entry::Vacant(_) => {
                clear_search_state(state);
                j += window;
                i = j;
            }
        }
    }
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() {
            return vec![];
        }
        let mut state: HashMap<&str, WordSearch> =
            words
                .iter()
                .fold(HashMap::with_capacity(words.len()), |mut acc, word| {
                    acc.entry(word)
                        .or_insert_with(WordSearch::default)
                        .inc_need();
                    acc
                });
        let window: usize = words[0].len();
        let count: usize = words.len();
        if window == 0 || s.len() < (window * count) {
            return vec![];
        }
        let mut out: BTreeSet<i32> = BTreeSet::new();
        for offset in 0..window {
            find_search_state(
                &s[offset..],
                window,
                offset,
                count,
                &mut state,
                |position| {
                    out.insert(position);
                },
            );
            clear_search_state(&mut state);
        }
        out.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0030() {
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                "barfoofoobarthefoobarman".to_string(),
                vec!["bar".to_string(), "foo".to_string(), "the".to_string()]
            ),
            vec![6, 9, 12]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring(
                "xxwordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![10]
        );
        assert_eq!(
            Solution::find_substring(
                "aaaaaaaa".to_string(),
                vec!["aa".to_string(), "aa".to_string(), "aa".to_string(),]
            ),
            vec![0, 1, 2]
        );
        assert_eq!(
            Solution::find_substring(
                std::iter::repeat('a').take(5000).collect(),
                std::iter::repeat_with(|| "a".to_string())
                    .take(5001)
                    .collect()
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                std::iter::successors(Some('a'), |&c| if c == 'a' { Some('b') } else { Some('a') })
                    .take(10000)
                    .collect(),
                std::iter::successors(Some("ab".to_string()), |s| if s == "ab" {
                    Some("ba".to_string())
                } else {
                    Some("ab".to_string())
                })
                .take(200)
                .collect(),
            ),
            vec![]
        );
    }
}
