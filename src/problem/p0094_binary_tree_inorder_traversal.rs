/**
 * [94] Binary Tree Inorder Traversal
 *
 * Given the root of a binary tree, return the inorder traversal of its nodes' values.
 *  
 * <strong class="example">Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">root = [1,null,2,3]</span>
 * Output: <span class="example-io">[1,3,2]</span>
 * Explanation:
 * <img alt="" src="https://assets.leetcode.com/uploads/2024/08/29/screenshot-2024-08-29-202743.png" style="width: 200px; height: 264px;" />
 * </div>
 * <strong class="example">Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">root = [1,2,3,4,5,null,8,null,null,6,7,9]</span>
 * Output: <span class="example-io">[4,2,6,5,7,1,3,9,8]</span>
 * Explanation:
 * <img alt="" src="https://assets.leetcode.com/uploads/2024/08/29/tree_2.png" style="width: 350px; height: 286px;" />
 * </div>
 * <strong class="example">Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">root = []</span>
 * Output: <span class="example-io">[]</span>
 * </div>
 * <strong class="example">Example 4:
 * <div class="example-block">
 * Input: <span class="example-io">root = [1]</span>
 * Output: <span class="example-io">[1]</span>
 * </div>
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 * 
 *  
 * Follow up: Recursive solution is trivial, could you do it iteratively?
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-inorder-traversal/
// discuss: https://leetcode.com/problems/binary-tree-inorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        Self::dfs(root, &mut result);
        result
    }
    
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>){
        if root.is_none() { return; }
        Self::dfs(root.as_ref().unwrap().borrow().left.clone(), result);
        result.push(root.as_ref().unwrap().borrow().val);
        Self::dfs(root.as_ref().unwrap().borrow().right.clone(), result);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_94() {
    }
}
