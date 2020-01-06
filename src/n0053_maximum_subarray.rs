/**
 * [0053] Maximum Subarray
 *
 * Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
 *
 * Example:
 *
 *
 * Input: [-2,1,-3,4,-1,2,1,-5,4],
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 *
 *
 * Follow up:
 *
 * If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
 *
 */
pub struct Solution {}

// submission codes start here

struct State {
    max: i32,
    sum: i32,
}

impl State {
    fn new() -> State {
        State {
            max: i32::min_value(),
            sum: 0,
        }
    }

    fn add(&mut self, num: i32) {
        self.sum += num;
        self.max = i32::max(self.max, self.sum);
        if self.sum < 0 {
            self.sum = 0;
        }
    }
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(State::new(), |mut state, num| {
                state.add(num);
                state
            })
            .max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0053() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
        assert_eq!(-8, Solution::max_sub_array(vec![-8]));
        assert_eq!(-2, Solution::max_sub_array(vec![-8, -2]));
    }
}
