/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 * Example:
 *
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        let mut list = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut list;
        loop {
            if l1.is_none() && l2.is_none() && carry == 0 {
                return list.unwrap().next;
            }
            let l1v = match l1 {
                Some(list_node) => {
                    l1 = list_node.next;
                    list_node.val
                }
                None => 0,
            };
            let l2v = match l2 {
                Some(list_node) => {
                    l2 = list_node.next;
                    list_node.val
                }
                None => 0,
            };
            let sum = match l1v + l2v + carry {
                sum if sum >= 10 => {
                    carry = 1;
                    sum - 10
                }
                sum => {
                    carry = 0;
                    sum
                }
            };
            let mut tail_node = tail.as_mut().unwrap();
            tail_node.next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail_node.next;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}
