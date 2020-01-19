/**
 * [0482] License Key Formatting
 *
 * You are given a license key represented as a string S which consists only alphanumeric character and dashes. The string is separated into N+1 groups by N dashes.
 *
 * Given a number K, we would want to reformat the strings such that each group contains exactly K characters, except for the first group which could be shorter than K, but still must contain at least one character. Furthermore, there must be a dash inserted between two groups and all lowercase letters should be converted to uppercase.
 *
 * Given a non-empty string S and a number K, format the string according to the rules described above.
 *
 * Example 1:
 *
 * Input: S = "5F3Z-2e-9-w", K = 4
 *
 * Output: "5F3Z-2E9W"
 *
 * Explanation: The string S has been split into two parts, each part has 4 characters.
 * Note that the two extra dashes are not needed and can be removed.
 *
 *
 *
 *
 * Example 2:
 *
 * Input: S = "2-5g-3-J", K = 2
 *
 * Output: "2-5G-3J"
 *
 * Explanation: The string S has been split into three parts, each part has 2 characters except the first part as it could be shorter as mentioned above.
 *
 *
 *
 * Note:
 *
 * The length of string S will not exceed 12,000, and K is a positive integer.
 * String S consists only of alphanumerical characters (a-z and/or A-Z and/or 0-9) and dashes(-).
 * String S is non-empty.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let rs: String = s
            .chars()
            .filter(|&c| c != '-')
            .map(|c| c.to_ascii_uppercase())
            .rev()
            .collect();
        rs.as_bytes()
            .chunks(k as usize)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect::<Vec<&str>>()
            .join("-")
            .chars()
            .rev()
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0482() {
        assert_eq!(
            "5F3Z-2E9W".to_string(),
            Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4)
        );
        assert_eq!(
            "2-5G-3J".to_string(),
            Solution::license_key_formatting("2-5g-3-J".to_string(), 2)
        );
    }
}
