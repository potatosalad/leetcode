/**
 * [31] Next Permutation
 *
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 *
 * If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).
 *
 * The replacement must be <a href="http://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> and use only constant extra memory.
 *
 * Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
 *
 * 1,2,3 &rarr; 1,3,2<br />
 * 3,2,1 &rarr; 1,2,3<br />
 * 1,1,5 &rarr; 1,5,1
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let digits = nums.len();
        if let Some(i) = (1..digits).rev().find(|&i| nums[i - 1] < nums[i]) {
            let j = (i..digits).rev().find(|&j| nums[i - 1] < nums[j]).unwrap();
            nums.swap(i - 1, j);
            nums[i..].reverse();
        } else {
            nums.reverse();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0031() {
        let mut vec1 = vec![1, 2, 3, 4, 5];
        Solution::next_permutation(&mut vec1);
        assert_eq!(vec1, vec![1, 2, 3, 5, 4]);

        let mut vec2 = vec![5, 4, 3, 2, 1];
        Solution::next_permutation(&mut vec2);
        assert_eq!(vec2, vec![1, 2, 3, 4, 5]);

        let mut vec3 = vec![1, 3, 2];
        Solution::next_permutation(&mut vec3);
        assert_eq!(vec3, vec![2, 1, 3]);
    }
}
