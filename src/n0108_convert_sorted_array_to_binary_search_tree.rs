/**
 * [0108] Convert Sorted Array to Binary Search Tree
 *
 * Given an array where elements are sorted in ascending order, convert it to a height balanced BST.
 *
 * For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
 *
 * Example:
 *
 *
 * Given the sorted array: [-10,-3,0,5,9],
 *
 * One possible answer is: [0,-3,9,-10,null,5], which represents the following height balanced BST:
 *
 *       0
 *      / \
 *    -3   9
 *    /   /
 *  -10  5
 *
 *
 */
pub struct Solution {}
pub use super::util::tree::{to_tree, TreeNode};

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            None
        } else {
            let median_index = nums.len() / 2;
            let root = Rc::new(RefCell::new(TreeNode::new(nums[median_index])));
            root.borrow_mut().left =
                Solution::sorted_array_to_bst((&nums[..median_index]).to_vec());
            root.borrow_mut().right =
                Solution::sorted_array_to_bst((&nums[median_index + 1..]).to_vec());
            Some(root)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0108() {
        assert_eq!(
            tree![0, -3, 9, -10, null, 5],
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
        );
    }
}
