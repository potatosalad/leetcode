/**
 * [0232] Implement Queue using Stacks
 *
 * Implement the following operations of a queue using stacks.
 *
 *
 * 	push(x) -- Push element x to the back of queue.
 * 	pop() -- Removes the element from in front of queue.
 * 	peek() -- Get the front element.
 * 	empty() -- Return whether the queue is empty.
 *
 *
 * Example:
 *
 *
 * MyQueue queue = new MyQueue();
 *
 * queue.push(1);
 * queue.push(2);  
 * queue.peek();  // returns 1
 * queue.pop();   // returns 1
 * queue.empty(); // returns false
 *
 * Notes:
 *
 *
 * 	You must use only standard operations of a stack -- which means only push to top, peek/pop from top, size, and is empty operations are valid.
 * 	Depending on your language, stack may not be supported natively. You may simulate a stack by using a list or deque (double-ended queue), as long as you use only standard operations of a stack.
 * 	You may assume that all operations are valid (for example, no pop or peek operations will be called on an empty queue).
 *
 *
 */
pub struct Solution {}

// submission codes start here

pub struct MyQueue {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        MyQueue { data: Vec::new() }
    }

    /** Push element x to the back of queue. */
    pub fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    pub fn pop(&mut self) -> i32 {
        self.data.remove(0)
    }

    /** Get the front element. */
    pub fn peek(&self) -> i32 {
        self.data[0]
    }

    /** Returns whether the queue is empty. */
    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0232() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(1, queue.peek());
        assert_eq!(1, queue.pop());
        assert!(!queue.empty());
        assert_eq!(2, queue.peek());
        assert_eq!(2, queue.pop());
        assert!(queue.empty());
    }
}
