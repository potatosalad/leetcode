/**
 * [88] Merge Sorted Array
 *
 * Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.
 *
 * Note:
 *
 *
 * 	The number of elements initialized in nums1 and nums2 are m and n respectively.
 * 	You may assume that nums1 has enough space (size that is greater or equal to m + n) to hold additional elements from nums2.
 *
 *
 * Example:
 *
 *
 * Input:
 * nums1 = [1,2,3,0,0,0], m = 3
 * nums2 = [2,5,6],       n = 3
 *
 * Output: [1,2,2,3,5,6]
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m_plus_n = m + n;
        let mut m = m;
        for i in 0..n {
            let mut filled = false;
            let num2 = nums2[i as usize];
            nums2[i as usize] = 0;
            for j in 0..m {
                let num1 = nums1[j as usize];
                if num1 >= num2 {
                    for k in ((j + 1)..m_plus_n).rev() {
                        nums1[k as usize] = nums1[(k - 1) as usize];
                    }
                    nums1[j as usize] = num2;
                    filled = true;
                    break;
                }
            }
            if filled {
                m += 1;
                continue;
            } else {
                nums1[m as usize] = num2;
                m += 1;
                continue;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut vec1 = vec![1, 2, 3, 0, 0, 0];
        let mut vec2 = vec![2, 5, 6];
        Solution::merge(&mut vec1, 3, &mut vec2, 3);
        assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);

        let mut vec1 = vec![1, 2, 3];
        let mut vec2 = vec![];
        Solution::merge(&mut vec1, 3, &mut vec2, 0);
        assert_eq!(vec1, vec![1, 2, 3]);

        let mut vec1 = vec![0, 0, 0];
        let mut vec2 = vec![1, 2, 3];
        Solution::merge(&mut vec1, 0, &mut vec2, 3);
        assert_eq!(vec1, vec![1, 2, 3]);
    }
}
