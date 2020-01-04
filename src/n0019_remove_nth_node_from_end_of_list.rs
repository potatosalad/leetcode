/**
 * [19] Remove Nth Node From End of List
 *
 * Given a linked list, remove the n-th node from the end of list and return its head.
 *
 * Example:
 *
 *
 * Given linked list: 1->2->3->4->5, and n = 2.
 *
 * After removing the second node from the end, the linked list becomes 1->2->3->5.
 *
 *
 * Note:
 *
 * Given n will always be valid.
 *
 * Follow up:
 *
 * Could you do this in one pass?
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut vs: Vec<i32> = Vec::new();
        if n < 0 {
            unreachable!()
        } else if let Some(mut node) = head {
            vs.push(node.val);
            while let Some(n) = node.next {
                node = n;
                vs.push(node.val);
            }
            if n as usize > vs.len() {
                unreachable!()
            } else {
                vs.remove(vs.len() - n as usize);
                let mut current = None;
                for &v in vs.iter().rev() {
                    let mut node = ListNode::new(v);
                    node.next = current;
                    current = Some(Box::new(node));
                }
                current
            }
        } else if n == 0 {
            None
        } else {
            unreachable!()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
}
