/**
 * [0450] Delete Node in a BST
 *
 * Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.
 *
 * Basically, the deletion can be divided into two stages:
 *
 * Search for a node to remove.
 * If the node is found, delete the node.
 *
 *
 *
 * Note: Time complexity should be O(height of tree).
 *
 * Example:
 *
 * root = [5,3,6,2,4,null,7]
 * key = 3
 *
 *     5
 *    / \
 *   3   6
 *  / \   \
 * 2   4   7
 *
 * Given key to delete is 3. So we find the node with value 3 and delete it.
 *
 * One valid answer is [5,4,6,2,null,null,7], shown in the following BST.
 *
 *     5
 *    / \
 *   4   6
 *  /     \
 * 2       7
 *
 * Another valid answer is [5,2,6,null,4,null,7].
 *
 *     5
 *    / \
 *   2   6
 *    \   \
 *     4   7
 *
 *
 */
pub struct Solution {}
pub use super::util::tree::{to_tree, TreeNode};

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

pub fn minimum_node(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut current = node;
    while let Some(n) = current.clone() {
        if n.borrow().left.is_none() {
            break;
        } else {
            current = n.borrow_mut().left.clone();
        }
    }
    current
}

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(n) => {
                if key < n.borrow_mut().val {
                    let new_left = Solution::delete_node(n.borrow_mut().left.clone(), key);
                    n.borrow_mut().left = new_left;
                } else if key > n.borrow_mut().val {
                    let new_right = Solution::delete_node(n.borrow_mut().right.clone(), key);
                    n.borrow_mut().right = new_right;
                } else if n.borrow_mut().left.is_none() {
                    let new_root = n.borrow_mut().right.clone();
                    n.borrow_mut().right = None;
                    return new_root;
                } else if n.borrow_mut().right.is_none() {
                    let new_root = n.borrow_mut().left.clone();
                    n.borrow_mut().left = None;
                    return new_root;
                } else {
                    let pivot_val = minimum_node(n.borrow_mut().right.clone())
                        .unwrap()
                        .borrow_mut()
                        .val;
                    n.borrow_mut().val = pivot_val;
                    let new_right = Solution::delete_node(n.borrow_mut().right.clone(), pivot_val);
                    n.borrow_mut().right = new_right;
                }
                Some(n)
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0450() {
        assert_eq!(
            tree![5, 4, 6, 2, null, null, 7],
            Solution::delete_node(tree![5, 3, 6, 2, 4, null, 7], 3)
        );
    }
}
