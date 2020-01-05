/**
 * [6] ZigZag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 *
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 *
 * string convert(string s, int numRows);
 *
 * Example 1:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 *
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let len: i32 = s.len() as i32;
        if num_rows < 2 || len < 3 {
            return s;
        }
        let bytes = s.as_bytes();
        let mut out = String::with_capacity(s.capacity());
        let cycle_every = num_rows * 2 - 2;
        for row in 0..num_rows {
            let mut cycle = 0;
            while cycle + row < len {
                out.push(bytes[(cycle + row) as usize] as char);
                if row > 0 && row != num_rows - 1 && cycle + cycle_every - row < len {
                    out.push(bytes[(cycle + cycle_every - row) as usize] as char);
                }
                cycle += cycle_every;
            }
        }
        out
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0006() {
        assert_eq!(
            "PINALSIGYAHRPI",
            Solution::convert("PAYPALISHIRING".to_string(), 4)
        );
        assert_eq!(
            "PAHNAPLSIIGYIR",
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );
        assert_eq!("A", Solution::convert("A".to_string(), 1));
        assert_eq!("AY", Solution::convert("AY".to_string(), 2));
        assert_eq!(
            "Aaidoeswr,haenme,rtesqecouishtabrateaeaietedrcinwtgnrlloacsoajsmnsoucutoadodiiesplnrmiaodprs,ubroohreunefnttacneedhsmwynihrieto,iheeaalwnefrdutettpntainnwrdvdr.",
            Solution::convert("Apalindromeisaword,phrase,number,orothersequenceofunitsthatcanbereadthesamewayineitherdirection,withgeneralallowancesforadjustmentstopunctuationandworddividers.".to_string(), 2),
        );
        assert_eq!(
            "PAYPALISHIRING",
            Solution::convert("PAYPALISHIRING".to_string(), 14),
        );
        assert_eq!(
            "PAYPALISHIRIGN",
            Solution::convert("PAYPALISHIRING".to_string(), 13),
        );
        assert_eq!(
            "PNAIGYRPIAHLSI",
            Solution::convert("PAYPALISHIRING".to_string(), 7),
        );
    }
}
