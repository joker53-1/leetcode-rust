/**
 * [95] Unique Binary Search Trees II
 *
 * Given an integer n, return all the structurally unique BST's (binary search trees), which has exactly n nodes of unique values from 1 to n. Return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg" style="width: 600px; height: 148px;" />
 * Input: n = 3
 * Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 1
 * Output: [[1]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 8
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/unique-binary-search-trees-ii/
// discuss: https://leetcode.com/problems/unique-binary-search-trees-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 { 
            return vec![];
        }
        Self::generate_trees_inner(1, n)        
    }
    
    fn generate_trees_inner(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        if start > end {
            result.push(None);
            return result;
        }
        
        for i in start..=end {
            let left = Self::generate_trees_inner(start, i - 1);
            let right = Self::generate_trees_inner(i + 1, end);
            for l in left.iter() {
                for r in right.iter() {
                    let mut node = TreeNode::new(i);
                    node.left = l.clone();
                    node.right = r.clone();
                    result.push(Some(Rc::new(RefCell::new(node))));
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_95() {
    }
}
