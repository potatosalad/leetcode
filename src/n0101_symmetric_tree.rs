/**
 * [0101] Symmetric Tree
 *
 * Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).
 *
 * For example, this binary tree [1,2,2,3,4,4,3] is symmetric:
 *
 *
 *     1
 *    / \
 *   2   2
 *  / \ / \
 * 3  4 4  3
 *
 *
 *  
 *
 * But the following [1,2,2,null,3,null,3] is not:
 *
 *
 *     1
 *    / \
 *   2   2
 *    \   \
 *    3    3
 *
 *
 *  
 *
 * Note:
 * Bonus points if you could solve it both recursively and iteratively.
 *
 */
pub struct Solution {}
pub use super::util::tree::{to_tree, TreeNode};

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn is_mirror_of(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (None, _) | (_, None) => false,
            (Some(left), Some(right)) => {
                let left = left.borrow_mut();
                let right = right.borrow_mut();
                left.val == right.val
                    && Solution::is_mirror_of(left.right.clone(), right.left.clone())
                    && Solution::is_mirror_of(left.left.clone(), right.right.clone())
            }
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow_mut();
            Solution::is_mirror_of(node.left.clone(), node.right.clone())
        } else {
            true
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0101() {
        assert!(Solution::is_symmetric(tree![1, 2, 2, 3, 4, 4, 3]));
        assert!(!Solution::is_symmetric(tree![1, 2, 2, null, 3, null, 3]));
    }
}
