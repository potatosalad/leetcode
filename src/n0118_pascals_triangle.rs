/**
 * [0118] Pascal's Triangle
 *
 * Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.
 *
 * In Pascal's triangle, each number is the sum of the two numbers directly above it.
 *
 * Example:
 *
 *
 * Input: 5
 * Output:
 * [
 *      [1],
 *     [1,1],
 *    [1,2,1],
 *   [1,3,3,1],
 *  [1,4,6,4,1]
 * ]
 *
 *
 */
pub struct Solution {}

// submission codes start here

pub struct PascalsTriangle {
    row_count: i32,
}

impl PascalsTriangle {
    pub fn new(row_count: i32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<i32>> {
        match self.row_count {
            0 => vec![],
            1 => vec![vec![1]],
            _ => {
                let mut rows = vec![vec![1], vec![1, 1]];
                for i in 1..(self.row_count as usize - 1) {
                    let mut row = Vec::new();
                    for j in 0..=(i as usize + 1) {
                        match j {
                            0 => row.push(1),
                            _ if j == i + 1 => row.push(1),
                            _ => row.push(rows[i][j - 1] + rows[i][j]),
                        }
                    }
                    rows.push(row);
                }
                rows
            }
        }
    }
}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        PascalsTriangle::new(num_rows).rows()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0118() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
