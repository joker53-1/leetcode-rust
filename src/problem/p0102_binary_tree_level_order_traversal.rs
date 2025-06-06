/**
 * [102] Binary Tree Level Order Traversal
 *
 * Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[3],[9,20],[15,7]]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: root = [1]
 * Output: [[1]]
 * 
 * <strong class="example">Example 3:
 * 
 * Input: root = []
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 2000].
 * 	-1000 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-level-order-traversal/
// discuss: https://leetcode.com/problems/binary-tree-level-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // if root.is_none() { return vec![]; }
        // let mut q = VecDeque::new();
        // q.push_back(root);
        // let mut res = Vec::new();
        // while !q.is_empty() {
        //     let len = q.len();
        //     let mut level = Vec::new();
        //     for _ in 0..len {
        //         if let Some(node) = q.pop_front() {
        //             if let Some(c) = node {
        //                 q.push_back(c.borrow().left.clone());
        //                 q.push_back(c.borrow().right.clone());
        //                 level.push(c.borrow().val);
        //             }
        //         }
        //     }
        //     if !level.is_empty() {
        //         res.push(level);
        //     }
        // }
        // res

        let root = match root {
            Some(node) => node,
            None => return Vec::new(),
        };

        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut result = Vec::new();

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level = Vec::with_capacity(level_size);

            for _ in 0..level_size {
                let node = queue.pop_front().unwrap();

                // 只借用一次
                {
                    let node_ref = node.borrow();
                    current_level.push(node_ref.val);

                    if let Some(left) = node_ref.left.as_ref() {
                        queue.push_back(Rc::clone(left));
                    }
                    if let Some(right) = node_ref.right.as_ref() {
                        queue.push_back(Rc::clone(right));
                    }
                } // 借用在这里结束
            }

            result.push(current_level);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_102() {
    }
}
