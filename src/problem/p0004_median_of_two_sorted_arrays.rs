/**
 * [4] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 * The overall run time complexity should be O(log (m+n)).
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 *  
 * Constraints:
 *
 * 	nums1.length == m
 * 	nums2.length == n
 * 	0 <= m <= 1000
 * 	0 <= n <= 1000
 * 	1 <= m + n <= 2000
 * 	-10^6 <= nums1[i], nums2[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.com/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let size = nums1.len() + nums2.len();
        if size == 1 { return if nums1.len() == 1 { nums1[0] as f64 } else { nums2[0] as f64 }  }
        let mid = size / 2;
        let (mut n1, mut n2, mut i) = (0, 0, 0);
        let mut pre = -1;
        let mut cur = -1;
        while i <= mid && n1 < nums1.len() && n2 < nums2.len() {
            pre = cur;
            if nums1[n1] < nums2[n2] {
                cur = nums1[n1];
                n1 += 1
            } else {
                cur = nums2[n2];
                n2 += 1
            }
            i += 1;
        }
        while i <= mid && n1 < nums1.len() {
            pre = cur;
            cur = nums1[n1];
            n1 += 1;
            i += 1;
        }
        while i <= mid && n2 < nums2.len() {
            pre = cur;
            cur = nums2[n2];
            n2 += 1;
            i += 1;
        }
        if size % 2 == 0 {  (pre + cur) as f64/2.0  } else { cur as f64 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {}
}
