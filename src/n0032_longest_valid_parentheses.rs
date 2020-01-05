/**
 * [32] Longest Valid Parentheses
 *
 * Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.
 *
 * Example 1:
 *
 *
 * Input: "(()"
 * Output: 2
 * Explanation: The longest valid parentheses substring is "()"
 *
 *
 * Example 2:
 *
 *
 * Input: ")()())"
 * Output: 4
 * Explanation: The longest valid parentheses substring is "()()"
 *
 *
 */
pub struct Solution {}

// submission codes start here

#[inline]
fn max_stack<I>(input: I, target: char) -> i32
where
    I: std::iter::IntoIterator<Item = char>,
{
    let mut stack = 0;
    let mut max = 0;
    let mut i = 0;
    for (j, c) in input.into_iter().enumerate() {
        if c == target {
            stack += 1;
        } else if stack == 0 {
            i = j as i32 + 1;
        } else {
            stack -= 1;
            if stack == 0 {
                max = i32::max(max, j as i32 - i + 1);
            }
        }
    }
    max
}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        i32::max(max_stack(s.chars(), '('), max_stack(s.chars().rev(), ')'))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0032() {
        assert_eq!(4, Solution::longest_valid_parentheses(")()())".to_string()));
        assert_eq!(0, Solution::longest_valid_parentheses(")(".to_string()));
        assert_eq!(2, Solution::longest_valid_parentheses("(()".to_string()));
        assert_eq!(
            4,
            Solution::longest_valid_parentheses("(((((()()".to_string()),
        );
        assert_eq!(
            6,
            Solution::longest_valid_parentheses("((((((((()))".to_string()),
        );
        assert_eq!(2, Solution::longest_valid_parentheses("()".to_string()));
        assert_eq!(2, Solution::longest_valid_parentheses("()(()".to_string()));
        assert_eq!(
            10,
            Solution::longest_valid_parentheses(")()(((())))(".to_string()),
        );
        assert_eq!(
            2,
            Solution::longest_valid_parentheses("(()(((()".to_string()),
        );
        assert_eq!(0, Solution::longest_valid_parentheses("".to_string()));
        assert_eq!(
            10,
            Solution::longest_valid_parentheses(")()()()()(()()()()()(".to_string())
        );
        assert_eq!(
            4,
            Solution::longest_valid_parentheses("()(()()(()".to_string())
        );
    }
}
