/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 *
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * If the target is not found in the array, return [-1, -1].
 *
 * Example 1:
 *
 *
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 *
 * Example 2:
 *
 *
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 *
 */
pub struct Solution {}

// submission codes start here

fn search_range_of_slice(nums: &[i32], target: i32, offset: i32) -> Option<(i32, i32)> {
    use std::cmp::Ordering;
    if nums.is_empty() {
        None
    } else {
        let i = nums.len() / 2;
        match nums[i].cmp(&target) {
            Ordering::Less if i == 0 => None,
            Ordering::Less => search_range_of_slice(&nums[i..], target, offset + i as i32),
            Ordering::Equal => {
                let mut min = i;
                let mut max = i;
                while min > 0 && nums[min - 1] == target {
                    min -= 1;
                }
                while max + 1 < nums.len() && nums[max + 1] == target {
                    max += 1;
                }
                Some((offset + min as i32, offset + max as i32))
            }
            Ordering::Greater => search_range_of_slice(&nums[..i], target, offset),
        }
    }
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if let Some((first, last)) = search_range_of_slice(&nums, target, 0) {
            vec![first, last]
        } else {
            vec![-1, -1]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
    }
}
