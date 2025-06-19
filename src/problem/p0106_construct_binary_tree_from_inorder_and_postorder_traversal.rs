/**
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
 *
 * Given two integer arrays inorder and postorder where inorder is the inorder traversal of a binary tree and postorder is the postorder traversal of the same tree, construct and return the binary tree.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" style="width: 277px; height: 302px;" />
 * Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
 * Output: [3,9,20,null,null,15,7]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: inorder = [-1], postorder = [-1]
 * Output: [-1]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= inorder.length <= 3000
 * 	postorder.length == inorder.length
 * 	-3000 <= inorder[i], postorder[i] <= 3000
 * 	inorder and postorder consist of unique values.
 * 	Each value of postorder also appears in inorder.
 * 	inorder is guaranteed to be the inorder traversal of the tree.
 * 	postorder is guaranteed to be the postorder traversal of the tree.
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();
        for (i, v) in inorder.iter().enumerate() {
            map.insert(v, i);
        }
        Self::recur((postorder.len() - 1) as i32, 0 , (postorder.len() - 1) as i32, &postorder, &map)
    }

    fn recur(root: i32, left: i32, right: i32, postorder: &Vec<i32>, map: &HashMap<&i32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right { return None; }
        let mut node = TreeNode::new(postorder[root as usize]);
        let i = map.get(&postorder[root as usize]).unwrap();
        node.left = Self::recur(root - (right - (*i as i32) + 1), left, (i - 1) as i32, postorder, map );
        node.right = Self::recur(root - 1, (i + 1) as i32, right, postorder, map);
        Some(Rc::new(RefCell::new(node)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_106() {
    }
}
