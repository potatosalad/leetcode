/**
 * [0703] Kth Largest Element in a Stream
 *
 * Design a class to find the kth largest element in a stream. Note that it is the kth largest element in the sorted order, not the kth distinct element.
 *
 * Your KthLargest class will have a constructor which accepts an integer k and an integer array nums, which contains initial elements from the stream. For each call to the method KthLargest.add, return the element representing the kth largest element in the stream.
 *
 * Example:
 *
 *
 * int k = 3;
 * int[] arr = [4,5,8,2];
 * KthLargest kthLargest = new KthLargest(3, arr);
 * kthLargest.add(3);   // returns 4
 * kthLargest.add(5);   // returns 5
 * kthLargest.add(10);  // returns 5
 * kthLargest.add(9);   // returns 8
 * kthLargest.add(4);   // returns 8
 *
 *
 * Note:
 * You may assume that nums' length &ge; k-1 and k &ge; 1.
 *
 */
pub struct Solution {}

// submission codes start here

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    k: i32,
    k_heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut k_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for num in nums {
            k_heap.push(Reverse(num));
        }
        while k_heap.len() > k as usize {
            k_heap.pop();
        }
        KthLargest { k, k_heap }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.k_heap.push(Reverse(val));
        if self.k_heap.len() > self.k as usize {
            self.k_heap.pop();
        }
        let Reverse(num) = self.k_heap.peek().cloned().unwrap();
        num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0703() {
        let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(4, obj.add(3));
        assert_eq!(5, obj.add(5));
        assert_eq!(5, obj.add(10));
        assert_eq!(8, obj.add(9));
        assert_eq!(8, obj.add(4));
        let mut obj = KthLargest::new(1, vec![]);
        assert_eq!(-3, obj.add(-3));
        assert_eq!(-2, obj.add(-2));
        assert_eq!(-2, obj.add(-4));
        assert_eq!(0, obj.add(0));
        assert_eq!(4, obj.add(4));
    }
}
