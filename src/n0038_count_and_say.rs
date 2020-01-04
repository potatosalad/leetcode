/**
 * [38] Count and Say
 *
 * The count-and-say sequence is the sequence of integers with the first five terms as following:
 *
 * 1.     1
 * 2.     11
 * 3.     21
 * 4.     1211
 * 5.     111221
 *
 * 1 is read off as "one 1" or 11.<br />
 * 11 is read off as "two 1s" or 21.<br />
 * 21 is read off as "one 2, then one 1" or 1211.
 * Given an integer n where 1 &le; n &le; 30, generate the n^th term of the count-and-say sequence. You can do so recursively, in other words from the previous member read off the digits, counting the number of digits in groups of the same digit.
 * Note: Each term of the sequence of integers will be represented as a string.
 *  
 * Example 1:
 *
 * Input: 1
 * Output: "1"
 * Explanation: This is the base case.
 *
 * Example 2:
 *
 * Input: 4
 * Output: "1211"
 * Explanation: For n = 3 the term was "21" in which we have two groups "2" and "1", "2" can be read as "12" which means frequency = 1 and value = 2, the same way "1" is read as "11", so the answer is the concatenation of "12" and "11" which is "1211".
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    #![allow(clippy::char_lit_as_u8)]
    pub fn count_and_say(n: i32) -> String {
        if n < 1 || n > 30 {
            unreachable!()
        } else if n == 1 {
            "1".into()
        } else {
            let previous = Solution::count_and_say(n - 1);
            let mut chars = previous.chars();
            let mut last = chars.next().unwrap();
            let mut frequency: u8 = 1;
            let mut out = String::new();
            for c in chars {
                if c == last {
                    frequency += 1;
                } else {
                    out.push((frequency + '0' as u8) as char);
                    out.push(last);
                    last = c;
                    frequency = 1;
                }
            }
            out.push((frequency + '0' as u8) as char);
            out.push(last);
            out
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_38() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(2), "11");
        assert_eq!(Solution::count_and_say(3), "21");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}
