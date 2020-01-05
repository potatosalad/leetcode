/**
 * [10] Regular Expression Matching
 *
 * Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.
 *
 * '.' Matches any single character.
 * '*' Matches zero or more of the preceding element.
 *
 * The matching should cover the entire input string (not partial).
 * Note:
 *
 * 	s could be empty and contains only lowercase letters a-z.
 * 	p could be empty and contains only lowercase letters a-z, and characters like . or *.
 *
 * Example 1:
 *
 * Input:
 * s = "aa"
 * p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 * Example 2:
 *
 * Input:
 * s = "aa"
 * p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
 *
 * Example 3:
 *
 * Input:
 * s = "ab"
 * p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 *
 * Example 4:
 *
 * Input:
 * s = "aab"
 * p = "c*a*b"
 * Output: true
 * Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore, it matches "aab".
 *
 * Example 5:
 *
 * Input:
 * s = "mississippi"
 * p = "mis*is*p*."
 * Output: false
 *
 */
pub struct Solution {}

// submission codes start here

#[derive(Clone, Copy, Debug, PartialEq)]
enum RegexCharacter {
    Any,
    Exact(char),
    Glob,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum RegexToken {
    Single(RegexCharacter),
    ZeroOrMore(RegexCharacter),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum RegexMatch {
    Char,
    Glob,
    Skip,
    Fail,
}

impl RegexCharacter {
    fn parse(c: char) -> RegexCharacter {
        match c {
            '.' => RegexCharacter::Any,
            '*' => RegexCharacter::Glob,
            _ => RegexCharacter::Exact(c),
        }
    }

    fn is_match(self, c: char) -> bool {
        match self {
            RegexCharacter::Any => true,
            RegexCharacter::Exact(x) => c == x,
            RegexCharacter::Glob => unreachable!(),
        }
    }
}

impl RegexToken {
    fn parse(pattern: &str) -> Vec<RegexToken> {
        if pattern.is_empty() {
            return vec![];
        }
        let mut tokens: Vec<RegexToken> = Vec::with_capacity(pattern.len());
        let mut prev: Option<RegexCharacter> = None;
        let chars = pattern.chars().skip_while(|&c| c == '*');
        for c in chars {
            let nc = RegexCharacter::parse(c);
            if let Some(pc) = prev.take() {
                if nc == RegexCharacter::Glob {
                    if pc == RegexCharacter::Glob {
                        prev = Some(nc);
                    } else {
                        tokens.push(RegexToken::ZeroOrMore(pc));
                    }
                } else {
                    tokens.push(RegexToken::Single(pc));
                    prev = Some(nc);
                }
            } else {
                prev = Some(nc);
            }
        }
        if let Some(pc) = prev.take() {
            if pc != RegexCharacter::Glob {
                tokens.push(RegexToken::Single(pc));
            }
        }
        tokens
    }

    fn try_match(self, c: char) -> RegexMatch {
        match self {
            RegexToken::Single(rc) => {
                if rc.is_match(c) {
                    RegexMatch::Char
                } else {
                    RegexMatch::Fail
                }
            }
            RegexToken::ZeroOrMore(rc) => {
                if rc.is_match(c) {
                    RegexMatch::Glob
                } else {
                    RegexMatch::Skip
                }
            }
        }
    }

    fn is_zero_or_more(self) -> bool {
        match self {
            RegexToken::ZeroOrMore(_) => true,
            _ => false,
        }
    }
}

impl RegexMatch {
    fn is_match(chars: &[char], tokens: &[RegexToken]) -> bool {
        if tokens.is_empty() {
            return chars.is_empty();
        } else if chars.is_empty() {
            return tokens.iter().all(|t| t.is_zero_or_more());
        }
        let mut cidx = 0;
        let mut tidx = 0;
        loop {
            if tidx >= tokens.len() {
                return cidx >= chars.len();
            } else if cidx >= chars.len() {
                return (&tokens[tidx..]).iter().all(|t| t.is_zero_or_more());
            }
            let c = chars[cidx];
            let t = tokens[tidx];
            match t.try_match(c) {
                RegexMatch::Char => {
                    cidx += 1;
                    tidx += 1;
                    continue;
                }
                RegexMatch::Glob => {
                    let tail_token_count: usize = (&tokens[tidx..])
                        .iter()
                        .filter(|t| !t.is_zero_or_more())
                        .count();
                    if cidx + tail_token_count >= chars.len()
                        || (tidx + 1 < tokens.len()
                            && RegexMatch::is_match(&chars[cidx..], &tokens[(tidx + 1)..]))
                    {
                        tidx += 1;
                        continue;
                    } else {
                        cidx += 1;
                        continue;
                    }
                }
                RegexMatch::Skip => {
                    tidx += 1;
                    continue;
                }
                RegexMatch::Fail => {
                    return false;
                }
            }
        }
    }
}

impl Solution {
    pub fn is_match(string: String, pattern: String) -> bool {
        let tokens = RegexToken::parse(pattern.as_str());
        let chars: Vec<char> = string.chars().collect();
        RegexMatch::is_match(&chars, &tokens)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0010() {
        assert_eq!(false, Solution::is_match("aa".to_owned(), "a".to_owned()));
        assert_eq!(true, Solution::is_match("aa".to_owned(), "a*".to_owned()));
        assert_eq!(true, Solution::is_match("ab".to_owned(), ".*".to_owned()));
        assert_eq!(
            true,
            Solution::is_match("aab".to_owned(), "c*a*b".to_owned())
        );
        assert_eq!(
            false,
            Solution::is_match("mississippi".to_owned(), "mis*is*p*.".to_owned())
        );
        assert_eq!(
            true,
            Solution::is_match("aaaa".to_owned(), "a*a".to_owned())
        );
        assert_eq!(
            true,
            Solution::is_match("aaaqqbc".to_owned(), "****a*.*bc".to_owned())
        );
        assert_eq!(
            false,
            Solution::is_match("aaaqqqwkejwklwzbc".to_owned(), "****a*.*yy*bc".to_owned())
        );
        assert_eq!(
            true,
            Solution::is_match("aaaqqqwkejwklwzybc".to_owned(), "****a*.*yy*bc".to_owned())
        );
        assert_eq!(
            true,
            Solution::is_match(
                "aasdfasdfasdfasdfas".to_owned(),
                "aasdf.*asdf.*asdf.*asdf.*s".to_owned()
            )
        );
    }
}
