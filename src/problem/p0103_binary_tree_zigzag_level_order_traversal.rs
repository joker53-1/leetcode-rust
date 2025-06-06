/**
 * [103] Binary Tree Zigzag Level Order Traversal
 *
 * Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[3],[20,9],[15,7]]
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
 * 	-100 <= Node.val <= 100
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
// discuss: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let root = match root {
            Some(node) => node,
            None => return vec![],
        };
        let mut res = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut flag = false;
        while !queue.is_empty() {
            let len = queue.len();
            let mut level = VecDeque::with_capacity(len);
            for _ in 0..len {
                let node = queue.pop_front().unwrap();
                {
                    let node = node.borrow();
                    if !flag {
                        level.push_back(node.val);
                    } else { 
                        level.push_front(node.val);
                    }
                    if let Some(left) = node.left.as_ref() {
                        queue.push_back(Rc::clone(left));
                    }
                    if let Some(right) = node.right.as_ref() {
                        queue.push_back(Rc::clone(right));
                    }
                }
            }
            flag = !flag;
            res.push(level.into());
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_103() {
    }
}
