/**
 * [0965] Unique Email Addresses
 *
 * Every email consists of a local name and a domain name, separated by the @ sign.
 *
 * For example, in alice@leetcode.com, alice is the local name, and leetcode.com is the domain name.
 *
 * Besides lowercase letters, these emails may contain '.'s or '+'s.
 *
 * If you add periods ('.') between some characters in the local name part of an email address, mail sent there will be forwarded to the same address without dots in the local name.  For example, "alice.z@leetcode.com" and "alicez@leetcode.com" forward to the same email address.  (Note that this rule does not apply for domain names.)
 *
 * If you add a plus ('+') in the local name, everything after the first plus sign will be ignored. This allows certain emails to be filtered, for example m.y+name@email.com will be forwarded to my@email.com.  (Again, this rule does not apply for domain names.)
 *
 * It is possible to use both of these rules at the same time.
 *
 * Given a list of emails, we send one email to each address in the list.  How many different addresses actually receive mails?
 *
 *  
 *
 * <div>
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">["test.email+alex@leetcode.com","test.e.mail+bob.cathy@leetcode.com","testemail+david@lee.tcode.com"]</span>
 * Output: <span id="example-output-1">2</span>
 * <span>Explanation:</span><span> "</span><span id="example-input-1-1">testemail@leetcode.com" and "testemail@lee.tcode.com" </span>actually receive mails
 *
 *
 *  
 *
 * Note:
 *
 *
 * 	1 <= emails[i].length <= 100
 * 	1 <= emails.length <= 100
 * 	Each emails[i] contains exactly one '@' character.
 * 	All local and domain names are non-empty.
 * 	Local names do not start with a '+' character.
 *
 * </div>
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashSet;

fn normalize_email(s: &str) -> String {
    let parts: Vec<&str> = s.split('@').collect();
    if parts.len() != 2 {
        unreachable!();
    }
    parts[0]
        .chars()
        .filter(|&c| c != '.')
        .take_while(|&c| c != '+')
        .chain(std::iter::once('@'))
        .chain(parts[1].chars())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let set: HashSet<String> = emails
            .into_iter()
            .map(|email| normalize_email(email.as_str()))
            .collect();
        set.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0965() {
        assert_eq!(
            2,
            Solution::num_unique_emails(vec_string![
                "test.email+alex@leetcode.com",
                "test.e.mail+bob.cathy@leetcode.com",
                "testemail+david@lee.tcode.com"
            ])
        );
    }
}
