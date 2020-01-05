/**
 * [23] Merge k Sorted Lists
 *
 * Merge k sorted linked lists and return it as one sorted list. Analyze and describe its complexity.
 *
 * Example:
 *
 *
 * Input:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * Output: 1->1->2->3->4->4->5->6
 *
 *
 */
pub struct Solution {}
pub use super::util::linked_list::{to_list, ListNode};

// submission codes start here

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut list = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut list;
    loop {
        match (l1.take(), l2.take()) {
            (None, None) => return list.unwrap().next,
            (Some(l1node), None) => {
                let mut node = tail.as_mut().unwrap();
                node.next = Some(l1node);
                tail = &mut node.next;
            }
            (None, Some(l2node)) => {
                let mut node = tail.as_mut().unwrap();
                node.next = Some(l2node);
                tail = &mut node.next;
            }
            (Some(l1node), Some(l2node)) => {
                if l1node.val <= l2node.val {
                    let mut node = tail.as_mut().unwrap();
                    node.next = Some(Box::new(ListNode::new(l1node.val)));
                    tail = &mut node.next;
                    l1 = l1node.next;
                } else {
                    l1 = Some(l1node.clone());
                }
                if l1node.val >= l2node.val {
                    let mut node = tail.as_mut().unwrap();
                    node.next = Some(Box::new(ListNode::new(l2node.val)));
                    tail = &mut node.next;
                    l2 = l2node.next;
                } else {
                    l2 = Some(l2node.clone());
                }
            }
        }
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            None
        } else if lists.len() == 1 {
            lists[0].clone()
        } else {
            let mut out = merge_two_lists(lists[0].clone(), lists[1].clone());
            for list in &lists[2..] {
                out = merge_two_lists(out, list.clone());
            }
            out
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0023() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }
}
