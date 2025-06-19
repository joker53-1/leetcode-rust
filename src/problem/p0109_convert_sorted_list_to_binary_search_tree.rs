/**
 * [109] Convert Sorted List to Binary Search Tree
 *
 * Given the head of a singly linked list where elements are sorted in ascending order, convert it to a <span data-keyword="height-balanced">height-balanced</span> binary search tree.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/linked.jpg" style="width: 500px; height: 388px;" />
 * Input: head = [-10,-3,0,5,9]
 * Output: [0,-3,9,-10,null,5]
 * Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: head = []
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in head is in the range [0, 2 * 10^4].
 * 	-10^5 <= Node.val <= 10^5
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
// discuss: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    // pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     if head.is_none() {
    //         return None;
    //     }
    //     Self::dfs(&head, 0, (nums.len() - 1) as i32)
    // }
    // 
    // fn dfs(nums: &Option<Box<ListNode>>, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
    //     
    // }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_109() {
    }
}
