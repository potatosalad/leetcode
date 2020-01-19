/**
 * [0940] Fruit Into Baskets
 *
 * In a row of trees, the i-th tree produces fruit with type tree[i].
 *
 * You start at any tree of your choice, then repeatedly perform the following steps:
 *
 * 	Add one piece of fruit from this tree to your baskets.  If you cannot, stop.
 * 	Move to the next tree to the right of the current tree.  If there is no tree to the right, stop.
 *
 * Note that you do not have any choice after the initial choice of starting tree: you must perform step 1, then step 2, then back to step 1, then step 2, and so on until you stop.
 *
 * You have two baskets, and each basket can carry any quantity of fruit, but you want each basket to only carry one type of fruit each.
 *
 * What is the total amount of fruit you can collect with this procedure?
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: [1,2,1]
 * Output: 3
 * Explanation: We can collect [1,2,1].
 *
 *
 * Example 2:
 *
 *
 * Input: [0,1,2,2]
 * Output: 3
 * Explanation: We can collect [1,2,2].
 * If we started at the first tree, we would only collect [0, 1].
 *
 *
 * Example 3:
 *
 *
 * Input: [1,2,3,2,2]
 * Output: 4
 * Explanation: We can collect [2,3,2,2].
 * If we started at the first tree, we would only collect [1, 2].
 *
 *
 * Example 4:
 *
 *
 * Input: [3,3,3,1,2,1,1,2,3,3,4]
 * Output: 5
 * Explanation: We can collect [1,2,1,1,2].
 * If we started at the first tree or the eighth tree, we would only collect 4 fruits.
 *
 * Note:
 *
 * 	1 <= tree.length <= 40000
 * 	0 <= tree[i] < tree.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn total_fruit(tree: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut count: i32 = 0;
        let mut first: i32 = 0;
        let mut second: i32 = -1;
        for i in 0..(tree.len() as i32) {
            count += 1;
            if tree[i as usize] == tree[first as usize] {
                first = i;
            } else if second == -1 || tree[i as usize] == tree[second as usize] {
                second = i;
            } else {
                max = i32::max(count - 1, max);
                count = (first - second).abs() + 1;
                first = i - 1;
                second = i;
            }
        }
        i32::max(count, max)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0940() {
        assert_eq!(3, Solution::total_fruit(vec![1, 2, 1]));
        assert_eq!(3, Solution::total_fruit(vec![0, 1, 2, 2]));
        assert_eq!(4, Solution::total_fruit(vec![1, 2, 3, 2, 2]));
        assert_eq!(
            5,
            Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4])
        );
    }
}
