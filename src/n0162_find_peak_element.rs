/**
 * [0162] Find Peak Element
 *
 * A peak element is an element that is greater than its neighbors.
 *
 * Given an input array nums, where nums[i] &ne; nums[i+1], find a peak element and return its index.
 *
 * The array may contain multiple peaks, in that case return the index to any one of the peaks is fine.
 *
 * You may imagine that nums[-1] = nums[n] = -&infin;.
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,1]
 * Output: 2
 * Explanation: 3 is a peak element and your function should return the index number 2.
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,1,3,5,6,4]
 * Output: 1 or 5
 * Explanation: Your function can return either index number 1 where the peak element is 2,
 *              or index number 5 where the peak element is 6.
 *
 *
 * Note:
 *
 * Your solution should be in logarithmic complexity.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => -1,
            1 => 0,
            _ => {
                if nums[1] < nums[0] {
                    0
                } else if nums[nums.len() - 2] < nums[nums.len() - 1] {
                    (nums.len() - 1) as i32
                } else {
                    for i in 1..nums.len() - 1 {
                        if nums[i - 1] < nums[i] && nums[i] > nums[i + 1] {
                            return i as i32;
                        }
                    }
                    unreachable!()
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0162() {
        assert_eq!(2, Solution::find_peak_element(vec![1, 2, 3, 1]));
        assert_eq!(1, Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]));
    }
}
