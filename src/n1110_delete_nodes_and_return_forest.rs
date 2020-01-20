/**
 * [1110] Delete Nodes And Return Forest
 *
 * Given the root of a binary tree, each node in the tree has a distinct value.
 *
 * After deleting all nodes with a value in to_delete, we are left with a forest (a disjoint union of trees).
 *
 * Return the roots of the trees in the remaining forest.  You may return the result in any order.
 *
 *  
 * Example 1:
 *
 *
 *
 *
 * Input: root = [1,2,3,4,5,6,7], to_delete = [3,5]
 * Output: [[1,2,null,4],[6],[7]]
 *
 *
 *  
 * Constraints:
 *
 *
 * 	The number of nodes in the given tree is at most 1000.
 * 	Each node has a distinct value between 1 and 1000.
 * 	to_delete.length <= 1000
 * 	to_delete contains distinct values between 1 and 1000.
 *
 */
pub struct Solution {}
pub use super::util::tree::{to_tree, TreeNode};

// submission codes start here

use std::cell::RefCell;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut results: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        let deletions: HashSet<i32> = HashSet::from_iter(to_delete);
        fn process(
            node: Option<Rc<RefCell<TreeNode>>>,
            deletions: &HashSet<i32>,
            results: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match node {
                None => None,
                Some(n) => {
                    let new_left = process(n.borrow_mut().left.clone(), deletions, results);
                    let new_right = process(n.borrow_mut().right.clone(), deletions, results);
                    n.borrow_mut().left = new_left;
                    n.borrow_mut().right = new_right;
                    if !deletions.contains(&n.borrow_mut().val) {
                        return Some(n);
                    }
                    if n.borrow_mut().right.is_some() {
                        results.push(n.borrow_mut().right.clone());
                    }
                    if n.borrow_mut().left.is_some() {
                        results.push(n.borrow_mut().left.clone());
                    }
                    None
                }
            }
        }
        let root = process(root, &deletions, &mut results);
        if root.is_some() {
            results.push(root);
        }
        results.reverse();
        results
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1110() {
        assert_eq!(
            vec![tree![1, 2, null, 4], tree![6], tree![7]],
            Solution::del_nodes(tree![1, 2, 3, 4, 5, 6, 7], vec![3, 5])
        );
    }
}
