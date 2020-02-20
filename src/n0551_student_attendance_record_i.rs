/**
 * [0551] Student Attendance Record I
 *
 * You are given a string representing an attendance record for a student. The record only contains the following three characters:
 *
 *
 *
 * 'A' : Absent.
 * 'L' : Late.
 *  'P' : Present.
 *
 *
 *
 *
 * A student could be rewarded if his attendance record doesn't contain more than one 'A' (absent) or more than two continuous 'L' (late).    
 *
 * You need to return whether the student could be rewarded according to his attendance record.
 *
 * Example 1:
 *
 * Input: "PPALLP"
 * Output: True
 *
 *
 *
 * Example 2:
 *
 * Input: "PPALLL"
 * Output: False
 *
 *
 *
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent = false;
        let mut late = 0;
        for c in s.chars() {
            match c {
                'A' => {
                    if absent == true {
                        return false;
                    } else {
                        absent = true;
                        late = 0;
                    }
                }
                'L' => {
                    late += 1;
                    if late >= 3 {
                        return false;
                    }
                }
                'P' => {
                    late = 0;
                }
                _ => unreachable!(),
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0551() {
        assert!(Solution::check_record("PPALLP".to_string()));
        assert!(!Solution::check_record("PPALLL".to_string()));
    }
}
