/**
 * [21] Merge Two Sorted Lists
 *
 * Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.
 *
 * Example:
 *
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
 *
 *
 */
pub struct Solution {}
use super::util::linked_list::{to_list, ListNode};

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
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
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}
