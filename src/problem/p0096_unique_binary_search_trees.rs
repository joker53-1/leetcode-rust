use std::cell::RefCell;
use std::rc::Rc;
use crate::util::tree::TreeNode;

/**
 * [96] Unique Binary Search Trees
 *
 * Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg" style="width: 600px; height: 148px;" />
 * Input: n = 3
 * Output: 5
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 1
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 19
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-binary-search-trees/
// discuss: https://leetcode.com/problems/unique-binary-search-trees/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // pub fn num_trees(n: i32) -> i32 {
    //     if n == 0 {
    //         return 0;
    //     }
    //     Self::generate_trees_inner(1, n)
    // }
    // 
    // fn generate_trees_inner(start: i32, end: i32) -> i32 {
    //     let mut ans = 0;
    //     if start > end {
    //         return 1;
    //     }
    // 
    //     for i in start..=end {
    //         let left = Self::generate_trees_inner(start, i - 1);
    //         let right = Self::generate_trees_inner(i + 1, end);
    //         ans += left * right;
    //     }
    //     ans
    // }
    pub fn num_trees(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut dp = vec![vec![0; n as usize]; n as usize];
        for i in 0..n as usize {
            dp[i][i] = 1;
        }
        for l in 1..n as usize {
            for i in 0..n as usize {
                if i+l >= n as usize { continue; }
                for j in i..=i + l {
                    let left = if j as i32 -1 < i as i32 { 1 } else { dp[i][j-1] };
                    let right = if j+1> i+l { 1 } else { dp[j+1][i+l] };
                    dp[i][i+l] += left * right;
                }
            }
        }
        dp[0][(n-1) as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_96() {
        print!("{}", Solution::num_trees(3));
    }
}
