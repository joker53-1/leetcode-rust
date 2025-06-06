/**
 * [100] Same Tree
 *
 * Given the roots of two binary trees p and q, write a function to check if they are the same or not.
 * Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/20/ex1.jpg" style="width: 622px; height: 182px;" />
 * Input: p = [1,2,3], q = [1,2,3]
 * Output: true
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/20/ex2.jpg" style="width: 382px; height: 182px;" />
 * Input: p = [1,2], q = [1,null,2]
 * Output: false
 * 
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/20/ex3.jpg" style="width: 622px; height: 182px;" />
 * Input: p = [1,2,1], q = [1,1,2]
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in both trees is in the range [0, 100].
 * 	-10^4 <= Node.val <= 10^4
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/same-tree/
// discuss: https://leetcode.com/problems/same-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() { return true; }
        if (p.is_none() && q.is_some()) || (p.is_some() && q.is_none()) { return false;  }
        Self::is_same_tree(p.as_ref().unwrap().borrow().left.clone(), q.as_ref().unwrap().borrow().left.clone()) && Self::is_same_tree(p.as_ref().unwrap().borrow().right.clone(), q.as_ref().unwrap().borrow().right.clone()) && p.unwrap().borrow().val == q.unwrap().borrow().val 
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_100() {
    }
}
