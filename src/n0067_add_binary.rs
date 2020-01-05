/**
 * [67] Add Binary
 *
 * Given two binary strings, return their sum (also a binary string).
 *
 * The input strings are both non-empty and contains only characters 1 or 0.
 *
 * Example 1:
 *
 *
 * Input: a = "11", b = "1"
 * Output: "100"
 *
 * Example 2:
 *
 *
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut xs = a.chars().rev().map(|c| c.to_digit(10).unwrap());
        let mut ys = b.chars().rev().map(|c| c.to_digit(10).unwrap());
        let mut zs: Vec<char> = Vec::with_capacity(1 + usize::max(a.len(), b.len()));
        let mut carry = 0;
        loop {
            let xnext = xs.next();
            let ynext = ys.next();
            if xnext.is_none() && ynext.is_none() {
                break;
            } else {
                let x = xnext.unwrap_or(0);
                let y = ynext.unwrap_or(0);
                let mut sum = x + y + carry;
                if sum > 1 {
                    sum -= 2;
                    carry = 1;
                } else {
                    carry = 0;
                }
                zs.push(std::char::from_digit(sum, 10).unwrap());
            }
        }
        if carry > 0 {
            zs.push('1');
        }
        zs.reverse();
        zs.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_67() {
        assert_eq!(
            "0".to_owned(),
            Solution::add_binary("0".to_owned(), "0".to_owned()),
        );
        assert_eq!(
            "10101".to_owned(),
            Solution::add_binary("1010".to_owned(), "1011".to_owned()),
        );
        assert_eq!(
            "100".to_owned(),
            Solution::add_binary("11".to_owned(), "1".to_owned()),
        );
        assert_eq!(
            "11110".to_owned(),
            Solution::add_binary("1111".to_owned(), "1111".to_owned()),
        );
    }
}
