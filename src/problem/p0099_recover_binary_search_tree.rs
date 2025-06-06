/**
 * [99] Recover Binary Search Tree
 *
 * You are given the root of a binary search tree (BST), where the values of exactly two nodes of the tree were swapped by mistake. Recover the tree without changing its structure.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/28/recover1.jpg" style="width: 422px; height: 302px;" />
 * Input: root = [1,3,null,null,2]
 * Output: [3,1,null,null,2]
 * Explanation: 3 cannot be a left child of 1 because 3 > 1. Swapping 1 and 3 makes the BST valid.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/28/recover2.jpg" style="width: 581px; height: 302px;" />
 * Input: root = [3,1,4,null,null,2]
 * Output: [2,1,4,null,null,3]
 * Explanation: 2 cannot be in the right subtree of 3 because 2 < 3. Swapping 2 and 3 makes the BST valid.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [2, 1000].
 * 	-2^31 <= Node.val <= 2^31 - 1
 * 
 *  
 * Follow up: A solution using O(n) space is pretty straight-forward. Could you devise a constant O(1) space solution?
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/recover-binary-search-tree/
// discuss: https://leetcode.com/problems/recover-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut res = vec![];
        Self::dfs(root, &mut res);
        let (mut x, mut y) = (None, None);
        for i in 0..res.len()-1 {
            if res[i+1] < res[i] { 
                y = Some(res[i+1]);
                if x == None { 
                    x = Some(res[i]);
                }
            }
        }
        Self::recover_tree2(root, x.unwrap(), y.unwrap());
    }
    
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = node {
            Self::dfs(&node.borrow().left, res);
            res.push(node.borrow().val);
            Self::dfs(&node.borrow().right, res);
        }
    }
    
    fn recover_tree2(root: &mut Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) {
        if let Some(node) = root { 
            Self::recover_tree2(&mut node.borrow_mut().left, x, y);
            if node.borrow().val == x || node.borrow().val == y { 
                node.borrow_mut().val = if node.borrow().val == x { y } else { x };
            }
            Self::recover_tree2(&mut node.borrow_mut().right, x, y);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_99() {
    }
}
