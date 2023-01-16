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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recur(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if l.is_none() && r.is_none() {
                true
            } else if l.is_some() && r.is_some() {
                let l_node = l.unwrap();
                let r_node = r.unwrap();
                let l_ref = l_node.borrow();
                let r_ref = r_node.borrow();
                l_ref.val == r_ref.val
                    && recur(l_ref.left.clone(), r_ref.right.clone())
                    && recur(l_ref.right.clone(), r_ref.left.clone())
            } else {
                false
            }
        }
        if let Some(node) = root {
            let node_ref = node.borrow();
            recur(node_ref.left.clone(), node_ref.right.clone())
        } else {
            true
        }
    }
}
