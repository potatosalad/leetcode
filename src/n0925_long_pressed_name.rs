/**
 * [0925] Long Pressed Name
 *
 * Your friend is typing his name into a keyboard.  Sometimes, when typing a character c, the key might get long pressed, and the character will be typed 1 or more times.
 * You examine the typed characters of the keyboard.  Return True if it is possible that it was your friends name, with some characters (possibly none) being long pressed.
 *  
 * Example 1:
 *
 * Input: name = "alex", typed = "aaleex"
 * Output: true
 * Explanation: 'a' and 'e' in 'alex' were long pressed.
 *
 * Example 2:
 *
 * Input: name = "saeed", typed = "ssaaedd"
 * Output: false
 * Explanation: 'e' must have been pressed twice, but it wasn't in the typed output.
 *
 * Example 3:
 *
 * Input: name = "leelee", typed = "lleeelee"
 * Output: true
 *
 * Example 4:
 *
 * Input: name = "laiden", typed = "laiden"
 * Output: true
 * Explanation: It's not necessary to long press any character.
 *
 *  
 * Constraints:
 *
 * 	1 <= name.length <= 1000
 * 	1 <= typed.length <= 1000
 * 	The characters of name and typed are lowercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name = name.as_bytes();
        let typed = typed.as_bytes();
        let m = name.len();
        let n = typed.len();
        let mut i = 0;
        for j in 0..n {
            if i < m && name[i] == typed[j] {
                i += 1;
            } else if j == 0 || typed[j] != typed[j - 1] {
                return false;
            }
        }
        i == m
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0925() {
        assert_eq!(
            true,
            Solution::is_long_pressed_name("alex".into(), "aaleex".into())
        );
        assert_eq!(
            false,
            Solution::is_long_pressed_name("saeed".into(), "ssaaedd".into())
        );
        assert_eq!(
            true,
            Solution::is_long_pressed_name("leelee".into(), "lleeelee".into())
        );
        assert_eq!(
            true,
            Solution::is_long_pressed_name("laiden".into(), "laiden".into())
        );
    }
}
