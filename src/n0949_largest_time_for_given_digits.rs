/**
 * [0949] Largest Time for Given Digits
 *
 * Given an array of 4 digits, return the largest 24 hour time that can be made.
 *
 * The smallest 24 hour time is 00:00, and the largest is 23:59.  Starting from 00:00, a time is larger if more time has elapsed since midnight.
 *
 * Return the answer as a string of length 5.  If no valid time can be made, return an empty string.
 *
 *  
 *
 *
 * Example 1:
 *
 *
 * Input: [1,2,3,4]
 * Output: "23:41"
 *
 *
 *
 * Example 2:
 *
 *
 * Input: [5,5,5,5]
 * Output: ""
 *
 *
 *  
 *
 * Note:
 *
 *
 * 	A.length == 4
 * 	0 <= A[i] <= 9
 *
 *
 *
 */
pub struct Solution {}

// submission codes start here

const FOUR_DIGIT_PERMUTATIONS: [[usize; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [2, 0, 1, 3],
    [2, 0, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [1, 2, 0, 3],
    [1, 2, 3, 0],
    [2, 1, 0, 3],
    [2, 1, 3, 0],
    [1, 3, 0, 2],
    [1, 3, 2, 0],
    [3, 1, 0, 2],
    [3, 1, 2, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0],
];

struct FourDigitPermutations {
    digits: Vec<i32>,
    index: usize,
}

impl FourDigitPermutations {
    fn new(digits: Vec<i32>) -> Self {
        Self { digits, index: 0 }
    }
}

impl Iterator for FourDigitPermutations {
    type Item = [i32; 4];
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 24 {
            let item = [
                self.digits[FOUR_DIGIT_PERMUTATIONS[self.index][0]].clone(),
                self.digits[FOUR_DIGIT_PERMUTATIONS[self.index][1]].clone(),
                self.digits[FOUR_DIGIT_PERMUTATIONS[self.index][2]].clone(),
                self.digits[FOUR_DIGIT_PERMUTATIONS[self.index][3]].clone(),
            ];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        let mut possibilities: Vec<[i32; 4]> = Vec::new();
        for [a, b, c, d] in FourDigitPermutations::new(a) {
            let f1 = a * 10 + b;
            let f2 = c * 10 + d;
            if f1 < 24 && f2 < 60 {
                possibilities.push([a, b, c, d]);
            }
        }
        possibilities.sort_unstable();
        possibilities.pop().map_or_else(
            || "".to_string(),
            |[a, b, c, d]| format!("{}{}:{}{}", a, b, c, d),
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0949() {
        assert_eq!(
            "23:41".to_string(),
            Solution::largest_time_from_digits(vec![1, 2, 3, 4])
        );
        assert_eq!(
            "".to_string(),
            Solution::largest_time_from_digits(vec![5, 5, 5, 5])
        );
    }
}
