/**
 * [4] Median of Two Sorted Arrays
 *
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.
 *
 * Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
 *
 * You may assume nums1 and nums2 cannot be both empty.
 *
 * Example 1:
 *
 *
 * nums1 = [1, 3]
 * nums2 = [2]
 *
 * The median is 2.0
 *
 *
 * Example 2:
 *
 *
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 *
 * The median is (2 + 3)/2 = 2.5
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let m_plus_n = m + n;
        let m_plus_n_median = m_plus_n / 2;
        let mut i = 0;
        let mut j = 0;
        let mut prev = 0;
        let mut next = 0;
        while (i + j) <= m_plus_n_median {
            let num1 = nums1.get(i).unwrap_or(&std::i32::MAX);
            let num2 = nums2.get(j).unwrap_or(&std::i32::MAX);
            if num1 < num2 {
                i += 1;
                prev = next;
                next = *num1;
            } else {
                j += 1;
                prev = next;
                next = *num2;
            }
        }
        if m_plus_n % 2 == 0 {
            f64::from(prev + next) / 2.0
        } else {
            f64::from(next)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_0004() {
        assert_approx_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_approx_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
