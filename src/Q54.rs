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
    pub fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn find(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> i32 {
            match root {
                Some(node) => {
                    let mut node_ref = node.borrow_mut();
                    let ret = find(node_ref.right.take(), k);
                    if *k == 0 {
                        return ret;
                    }
                    *k -= 1;
                    if *k == 0 {
                        return node_ref.val;
                    }
                    return find(node_ref.left.take(), k);
                }
                None => return 0,
            }
        }
        let mut k = k;
        find(root, &mut k)
    }
}
