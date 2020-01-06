/**
 * [0044] Wildcard Matching
 *
 * Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*'.
 *
 *
 * '?' Matches any single character.
 * '*' Matches any sequence of characters (including the empty sequence).
 *
 *
 * The matching should cover the entire input string (not partial).
 *
 * Note:
 *
 *
 * 	s could be empty and contains only lowercase letters a-z.
 * 	p could be empty and contains only lowercase letters a-z, and characters like ? or *.
 *
 *
 * Example 1:
 *
 *
 * Input:
 * s = "aa"
 * p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 *
 * Example 2:
 *
 *
 * Input:
 * s = "aa"
 * p = "*"
 * Output: true
 * Explanation: '*' matches any sequence.
 *
 *
 * Example 3:
 *
 *
 * Input:
 * s = "cb"
 * p = "?a"
 * Output: false
 * Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
 *
 *
 * Example 4:
 *
 *
 * Input:
 * s = "adceb"
 * p = "*a*b"
 * Output: true
 * Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".
 *
 *
 * Example 5:
 *
 *
 * Input:
 * s = "acdcb"
 * p = "a*c?b"
 * Output: false
 *
 *
 */
pub struct Solution {}

// submission codes start here

// #[derive(Clone, Copy, Debug, PartialEq)]
// enum WildcardCharacter {
//     Any,
//     Exact(char),
//     Glob,
// }

// #[derive(Clone, Copy, Debug, PartialEq)]
// enum WildcardToken {
//     Single(WildcardCharacter),
//     ZeroOrMore,
// }

// #[derive(Clone, Copy, Debug, PartialEq)]
// enum WildcardMatch {
//     Char,
//     Glob,
//     Fail,
// }

// impl WildcardCharacter {
//     fn parse(c: char) -> WildcardCharacter {
//         match c {
//             '?' => WildcardCharacter::Any,
//             '*' => WildcardCharacter::Glob,
//             _ => WildcardCharacter::Exact(c),
//         }
//     }

//     fn is_match(self, c: char) -> bool {
//         match self {
//             WildcardCharacter::Any => true,
//             WildcardCharacter::Exact(x) => c == x,
//             WildcardCharacter::Glob => unreachable!(),
//         }
//     }
// }

// impl From<WildcardCharacter> for WildcardToken {
//     fn from(wc: WildcardCharacter) -> WildcardToken {
//         match wc {
//             WildcardCharacter::Glob => WildcardToken::ZeroOrMore,
//             _ => WildcardToken::Single(wc),
//         }
//     }
// }

// impl WildcardToken {
//     fn parse(pattern: &str) -> Vec<WildcardToken> {
//         if pattern.is_empty() {
//             return vec![];
//         }
//         let raw_tokens: Vec<WildcardToken> = pattern
//             .chars()
//             .map(|c| WildcardToken::from(WildcardCharacter::parse(c)))
//             .collect();
//         let mut tokens: Vec<WildcardToken> = Vec::with_capacity(raw_tokens.len());
//         let mut prev_token = raw_tokens[0];
//         tokens.push(prev_token);
//         for &next_token in &raw_tokens[1..] {
//             if prev_token.is_zero_or_more() && prev_token == next_token {
//                 continue;
//             } else {
//                 prev_token = next_token;
//                 tokens.push(next_token);
//             }
//         }
//         tokens
//     }

//     fn try_match(self, c: char) -> WildcardMatch {
//         match self {
//             WildcardToken::Single(wc) => {
//                 if wc.is_match(c) {
//                     WildcardMatch::Char
//                 } else {
//                     WildcardMatch::Fail
//                 }
//             }
//             WildcardToken::ZeroOrMore => WildcardMatch::Glob,
//         }
//     }

//     fn is_zero_or_more(self) -> bool {
//         match self {
//             WildcardToken::ZeroOrMore => true,
//             _ => false,
//         }
//     }
// }

// impl WildcardMatch {
//     fn is_match(chars: &[char], tokens: &[WildcardToken]) -> bool {
//         if tokens.is_empty() {
//             return chars.is_empty();
//         } else if chars.is_empty() {
//             return tokens.iter().all(|t| t.is_zero_or_more());
//         }
//         let mut cidx = 0;
//         let mut tidx = 0;
//         loop {
//             if tidx >= tokens.len() {
//                 return cidx >= chars.len();
//             } else if cidx >= chars.len() {
//                 return (&tokens[tidx..]).iter().all(|t| t.is_zero_or_more());
//             }
//             let c = chars[cidx];
//             let t = tokens[tidx];
//             match t.try_match(c) {
//                 WildcardMatch::Char => {
//                     cidx += 1;
//                     tidx += 1;
//                     continue;
//                 }
//                 WildcardMatch::Glob => {
//                     let tail_token_count: usize = (&tokens[tidx..])
//                         .iter()
//                         .filter(|t| !t.is_zero_or_more())
//                         .count();
//                     if cidx + tail_token_count >= chars.len()
//                         || (tidx + 1 < tokens.len()
//                             && WildcardMatch::is_match(&chars[cidx..], &tokens[(tidx + 1)..]))
//                     {
//                         tidx += 1;
//                         continue;
//                     } else {
//                         cidx += 1;
//                         continue;
//                     }
//                 }
//                 WildcardMatch::Fail => {
//                     return false;
//                 }
//             }
//         }
//     }
// }

impl Solution {
    #![allow(clippy::char_lit_as_u8, clippy::many_single_char_names)]
    pub fn is_match(s: String, p: String) -> bool {
        let n = s.len();
        let m = p.len();
        if m == 0 {
            return n == 0;
        }
        let mut i = 0;
        let mut j = 0;
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut xs = None;
        let mut xp = None;
        while i < n {
            if j < m && (s[i] == p[j] || p[j] == '?' as u8) {
                i += 1;
                j += 1;
            } else if j < m && p[j] == '*' as u8 {
                xs = Some(i);
                xp = Some(j);
                j += 1;
            } else if let Some(xj) = xp.take() {
                let xi = xs.take().unwrap();
                j = xj + 1;
                i = xi + 1;
                xs = Some(xi + 1);
                xp = Some(xj);
            } else {
                return false;
            }
        }
        while j < m && p[j] == '*' as u8 {
            j += 1;
        }
        j == m
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0044() {
        let test_vectors: Vec<(bool, &'static str, &'static str)> = vec![
            (false, "aa", "a"),
            (true, "aa", "*"),
            (false, "cb", "?a"),
            (true, "adceb", "*a*b"),
            (false, "acdcb", "a*c?b"),
            (true, "aaaabaaaabbbbaabbbaabbaababbabbaaaababaaabbbbbbaabbbabababbaaabaabaaaaaabbaabbbbaababbababaabbbaababbbba", "*****b*aba***babaa*bbaba***a*aaba*b*aa**a*b**ba***a*a*"),
            (true, "abefcdgiescdfimde", "ab*cd?i*de"),
        ];
        for (i, &(valid, s, p)) in test_vectors.iter().enumerate() {
            assert!(
                valid == Solution::is_match(s.to_string(), p.to_string()),
                "expected test vector i={:?}, s={:?}, p={:?} to be {}",
                i,
                s,
                p,
                valid
            );
        }
    }
}
