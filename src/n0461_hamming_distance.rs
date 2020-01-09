/**
 * [0461] Hamming Distance
 *
 * The <a href="https://en.wikipedia.org/wiki/Hamming_distance" target="_blank">Hamming distance</a> between two integers is the number of positions at which the corresponding bits are different.
 *
 * Given two integers x and y, calculate the Hamming distance.
 *
 * Note:
 * 0 &le; x, y < 2^31.
 *
 *
 * Example:
 *
 * Input: x = 1, y = 4
 *
 * Output: 2
 *
 * Explanation:
 * 1   (0 0 0 1)
 * 4   (0 1 0 0)
 *        &uarr;   &uarr;
 *
 * The above arrows point to positions where the corresponding bits are different.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0461() {
        assert_eq!(2, Solution::hamming_distance(1, 4));
    }
}
