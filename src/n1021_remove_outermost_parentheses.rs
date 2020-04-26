/**
 * [1021] Remove Outermost Parentheses
 *
 * A valid parentheses string is either empty (""), "(" + A + ")", or A + B, where A and B are valid parentheses strings, and + represents string concatenation.  For example, "", "()", "(())()", and "(()(()))" are all valid parentheses strings.
 *
 * A valid parentheses string S is primitive if it is nonempty, and there does not exist a way to split it into S = A+B, with A and B nonempty valid parentheses strings.
 *
 * Given a valid parentheses string S, consider its primitive decomposition: S = P_1 + P_2 + ... + P_k, where P_i are primitive valid parentheses strings.
 *
 * Return S after removing the outermost parentheses of every primitive string in the primitive decomposition of S.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: "(()())(())"
 * Output: "()()()"
 * Explanation:
 * The input string is "(()())(())", with primitive decomposition "(()())" + "(())".
 * After removing outer parentheses of each part, this is "()()" + "()" = "()()()".
 *
 *
 *
 * Example 2:
 *
 *
 * Input: "(()())(())(()(()))"
 * Output: "()()()()(())"
 * Explanation:
 * The input string is "(()())(())(()(()))", with primitive decomposition "(()())" + "(())" + "(()(()))".
 * After removing outer parentheses of each part, this is "()()" + "()" + "()(())" = "()()()()(())".
 *
 *
 *
 * Example 3:
 *
 *
 * Input: "()()"
 * Output: ""
 * Explanation:
 * The input string is "()()", with primitive decomposition "()" + "()".
 * After removing outer parentheses of each part, this is "" + "" = "".
 *
 *
 *  
 *
 *
 *
 * Note:
 *
 *
 * 	S.length <= 10000
 * 	S[i] is "(" or ")"
 * 	S is a valid parentheses string
 *
 *
 *
 *
 *  
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut counter: i32 = 0;
        let mut output = String::new();
        for c in s.chars() {
            if counter != 0 && !(counter == 1 && c == ')') {
                output.push(c);
            }
            if c == '(' {
                counter += 1;
            } else {
                counter -= 1;
            }
        }
        output
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1021() {
        assert_eq!(
            "()()()".to_string(),
            Solution::remove_outer_parentheses("(()())(())".to_string())
        );
        assert_eq!(
            "()()()()(())".to_string(),
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::remove_outer_parentheses("()()".to_string())
        );
    }
}
