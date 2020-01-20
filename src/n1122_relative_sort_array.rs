/**
 * [1122] Relative Sort Array
 *
 * Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all elements in arr2 are also in arr1.
 * Sort the elements of arr1 such that the relative ordering of items in arr1 are the same as in arr2.  Elements that don't appear in arr2 should be placed at the end of arr1 in ascending order.
 *  
 * Example 1:
 * Input: arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
 * Output: [2,2,2,1,4,3,3,9,6,7,19]
 *  
 * Constraints:
 *
 * 	arr1.length, arr2.length <= 1000
 * 	0 <= arr1[i], arr2[i] <= 1000
 * 	Each arr2[i] is distinct.
 * 	Each arr2[i] is in arr1.
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let set: HashSet<i32> = arr2.iter().copied().collect();
        let mut histogram: HashMap<i32, usize> = HashMap::new();
        let mut tail: Vec<i32> = Vec::new();
        for n in arr1 {
            if set.contains(&n) {
                *(histogram.entry(n).or_insert(0)) += 1;
            } else {
                tail.push(n);
            }
        }
        let mut head: Vec<i32> = Vec::new();
        for n in arr2 {
            let x: usize = histogram.get(&n).copied().unwrap();
            head.append(&mut vec![n; x]);
        }
        tail.sort_unstable();
        head.append(&mut tail);
        head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1122() {
        assert_eq!(
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19],
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            )
        );
    }
}
