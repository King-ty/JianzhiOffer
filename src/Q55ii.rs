// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn solve(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            use std::cmp::max;
            match root {
                Some(node) => {
                    let mut node_ref = node.borrow_mut();
                    // let mut node_ref: RefMut<TreeNode> = node.borrow_mut();
                    let (deepl, deepr) =
                        (solve(node_ref.left.take()), solve(node_ref.right.take()));
                    if deepl != -1 && deepr != -1 && (deepl - deepr).abs() <= 1 {
                        return max(deepl, deepr) + 1;
                    } else {
                        return -1;
                    }
                }
                None => return 0,
            }
        }
        solve(root) != -1
    }
}
