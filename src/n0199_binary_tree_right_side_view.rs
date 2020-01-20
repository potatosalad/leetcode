/**
 * [0199] Binary Tree Right Side View
 *
 * Given a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
 *
 * Example:
 *
 *
 * Input: [1,2,3,null,5,null,4]
 * Output: [1, 3, 4]
 * Explanation:
 *
 *    1            <---
 *  /   \
 * 2     3         <---
 *  \     \
 *   5     4       <---
 *
 */
pub struct Solution {}
pub use super::util::tree::{to_tree, TreeNode};

// submission codes start here

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(n) => {
                let mut view: Vec<i32> = Vec::new();
                let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
                queue.push_back(n);
                while !queue.is_empty() {
                    let size = queue.len();
                    for i in 0..size {
                        let node = queue.pop_front().unwrap();
                        if i == 0 {
                            view.push(node.borrow_mut().val);
                        }
                        if node.borrow_mut().right.is_some() {
                            queue.push_back(node.borrow_mut().right.clone().unwrap());
                        }
                        if node.borrow_mut().left.is_some() {
                            queue.push_back(node.borrow_mut().left.clone().unwrap());
                        }
                    }
                }
                view
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0199() {
        assert_eq!(
            vec![1, 3, 4],
            Solution::right_side_view(tree![1, 2, 3, null, 5, null, 4])
        );
    }
}
