/**
 * [0119] Pascal's Triangle II
 *
 * Given a non-negative index k where k &le; 33, return the k^th index row of the Pascal's triangle.
 *
 * Note that the row index starts from 0.
 *
 * In Pascal's triangle, each number is the sum of the two numbers directly above it.
 *
 * Example:
 *
 *
 * Input: 3
 * Output: [1,3,3,1]
 *
 *
 * Follow up:
 *
 * Could you optimize your algorithm to use only O(k) extra space?
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut row = vec![1; row_index + 1];
        for i in 0..(row_index + 1) {
            let mut prev = 1;
            for j in 1..i {
                let temp = row[j];
                row[j] = temp + prev;
                prev = temp;
            }
        }
        row
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0119() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1])
    }
}
