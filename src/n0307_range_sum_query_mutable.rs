/**
 * [0307] Range Sum Query - Mutable
 *
 * Given an integer array nums, find the sum of the elements between indices i and j (i &le; j), inclusive.
 *
 * The update(i, val) function modifies nums by updating the element at index i to val.
 *
 * Example:
 *
 *
 * Given nums = [1, 3, 5]
 *
 * sumRange(0, 2) -> 9
 * update(1, 2)
 * sumRange(0, 2) -> 8
 *
 *
 * Note:
 *
 *
 * 	The array is only modifiable by the update function.
 * 	You may assume the number of calls to update and sumRange function is distributed evenly.
 *
 *
 */
pub struct Solution {}

// submission codes start here

pub struct NumArray {
    nums: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        NumArray { nums }
    }

    pub fn update(&mut self, i: i32, val: i32) {
        self.nums[i as usize] = val;
    }

    pub fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.nums[(i as usize)..=(j as usize)].iter().cloned().sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0307() {
        let mut obj = NumArray::new(vec![1, 3, 5]);
        assert_eq!(9, obj.sum_range(0, 2));
        obj.update(1, 2);
        assert_eq!(8, obj.sum_range(0, 2));
    }
}
