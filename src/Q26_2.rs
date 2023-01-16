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
    pub fn is_sub_structure(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn match_sub(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if b.is_none() {
                true
            } else if a.is_none() {
                false
            } else {
                a.clone().unwrap().borrow().val == b.clone().unwrap().borrow().val
                    && match_sub(
                        a.clone().unwrap().borrow().left.clone(),
                        b.clone().unwrap().borrow().left.clone(),
                    )
                    && match_sub(
                        a.clone().unwrap().borrow().right.clone(),
                        b.clone().unwrap().borrow().right.clone(),
                    )
            }
        }
        a.is_some()
            && b.is_some()
            && (match_sub(a.clone(), b.clone())
                || Self::is_sub_structure(a.clone().unwrap().borrow().left.clone(), b.clone())
                || Self::is_sub_structure(a.unwrap().borrow().right.clone(), b.clone()))
    }
}
