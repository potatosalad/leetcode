/**
 * [24] Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head.
 *
 * You may not modify the values in the list's nodes, only nodes itself may be changed.
 *
 *  
 *
 * Example:
 *
 *
 * Given 1->2->3->4, you should return the list as 2->1->4->3.
 *
 *
 */
pub struct Solution {}
pub use super::util::linked_list::{to_list, ListNode};

// submission codes start here

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = dummy_head.as_mut();
        loop {
            let mut lhs = head.as_mut().unwrap().next.take();
            if lhs.is_none() {
                break;
            }
            let mut rhs = lhs.as_mut().unwrap().next.take();
            if rhs.is_none() {
                head.as_mut().unwrap().next = lhs;
                break;
            }
            lhs.as_mut().unwrap().next = rhs.as_mut().unwrap().next.take();
            rhs.as_mut().unwrap().next = lhs;
            head.as_mut().unwrap().next = rhs;
            head = head.unwrap().next.as_mut().unwrap().next.as_mut();
        }
        dummy_head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24() {
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![])), to_list(vec![]));
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3])),
            to_list(vec![2, 1, 3])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![1])), to_list(vec![1]));
    }
}
