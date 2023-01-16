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
pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn match_sub(a: Rc<RefCell<TreeNode>>, b: Rc<RefCell<TreeNode>>) -> bool {
        let mut a_mut = a.borrow_mut();
        let mut b_mut = b.borrow_mut();
        if a_mut.val != b_mut.val {
            return false;
        }
        if let Some(b_left) = b_mut.left.take() {
            match a_mut.left.take() {
                Some(a_left) => {
                    if !Self::match_sub(a_left, b_left) {
                        return false;
                    }
                }
                None => return false,
            }
        }
        if let Some(b_right) = b_mut.right.take() {
            match a_mut.right.take() {
                Some(a_right) => {
                    return Self::match_sub(a_right, b_right);
                }
                None => return false,
            }
        }
        true
    }

    pub fn is_sub_structure(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match b {
            Some(node_b) => {
                if let Some(node_a) = a {
                    let node_a_mut = node_a.borrow_mut();
                    let node_b_mut = node_b.borrow();
                    let a_val = node_a_mut.val;
                    let b_val = node_b_mut.val;
                    drop(node_a_mut);
                    drop(node_b_mut);
                    if a_val == b_val {
                        if Self::match_sub(node_a.clone(), node_b.clone()) {
                            return true;
                        }
                    }
                    let mut node_a_mut = node_a.borrow_mut();
                    let node_a_left = node_a_mut.left.take();
                    let node_a_right = node_a_mut.right.take();
                    drop(node_a_mut);
                    if Self::is_sub_structure(node_a_left, Some(node_b.clone())) {
                        return true;
                    }
                    if Self::is_sub_structure(node_a_right, Some(node_b.clone())) {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}
