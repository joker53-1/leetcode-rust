/**
 * [101] Symmetric Tree
 *
 * Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree1.jpg" style="width: 354px; height: 291px;" />
 * Input: root = [1,2,2,3,4,4,3]
 * Output: true
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree2.jpg" style="width: 308px; height: 258px;" />
 * Input: root = [1,2,2,null,3,null,3]
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	-100 <= Node.val <= 100
 * 
 *  
 * Follow up: Could you solve it both recursively and iteratively?
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/symmetric-tree/
// discuss: https://leetcode.com/problems/symmetric-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() { return true; }
        Self::is_same_tree(root.as_ref().unwrap().borrow().left.clone(), root.as_ref().unwrap().borrow().right.clone())
    }
    
    fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() { return true; }
        if (p.is_none() && q.is_some()) || (p.is_some() && q.is_none()) { return false;  }
        Self::is_same_tree(p.as_ref().unwrap().borrow().left.clone(), q.as_ref().unwrap().borrow().right.clone()) && Self::is_same_tree(p.as_ref().unwrap().borrow().right.clone(), q.as_ref().unwrap().borrow().left.clone()) && p.unwrap().borrow().val == q.unwrap().borrow().val
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_101() {
    }
}
