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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            la_val: &mut i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root.as_ref() {
                let mut node_ref = node.borrow_mut();
                if node_ref.right.is_some() {
                    node_ref.right = dfs(node_ref.right.take(), la_val);
                }
                node_ref.val += *la_val;
                *la_val = node_ref.val;
                node_ref.left = dfs(node_ref.left.take(), la_val);
            }
            root
        }
        dfs(root, &mut 0)
    }
}
