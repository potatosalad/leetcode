/**
 * [83] Remove Duplicates from Sorted List
 *
 * Given a sorted linked list, delete all duplicates such that each element appear only once.
 *
 * Example 1:
 *
 *
 * Input: 1->1->2
 * Output: 1->2
 *
 *
 * Example 2:
 *
 *
 * Input: 1->1->2->3->3
 * Output: 1->2->3
 *
 *
 */
pub struct Solution {}
pub use super::util::linked_list::{to_list, ListNode};

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let mut val = node.val;
            let mut list = Some(Box::new(ListNode::new(node.val)));
            let mut tail = &mut list;
            while let Some(n) = node.next {
                node = n;
                if val != node.val {
                    let mut tail_node = tail.as_mut().unwrap();
                    val = node.val;
                    tail_node.next = Some(Box::new(ListNode::new(node.val)));
                    tail = &mut tail_node.next;
                }
            }
            list
        } else {
            None
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_83() {
        assert_eq!(
            to_list(vec![1, 2]),
            Solution::delete_duplicates(to_list(vec![1, 1, 2]))
        );
        assert_eq!(
            to_list(vec![1, 2, 3]),
            Solution::delete_duplicates(to_list(vec![1, 1, 2, 3, 3]))
        );
    }
}
