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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        fn find(
            root: Option<Rc<RefCell<TreeNode>>>,
            target: i32,
            paths: &mut Vec<Vec<i32>>,
            path: &mut Vec<i32>,
        ) {
            match root {
                Some(node) => {
                    let mut node_ref = node.borrow_mut();
                    path.push(node_ref.val);
                    if node_ref.left.is_none() && node_ref.right.is_none() {
                        if target == node_ref.val {
                            paths.push(path.clone());
                        }
                        path.pop();
                        return;
                    }
                    find(node_ref.left.take(), target - node_ref.val, paths, path);
                    find(node_ref.right.take(), target - node_ref.val, paths, path);
                    path.pop();
                }
                None => {
                    return;
                }
            }
        }
        find(root, target, &mut ret, &mut Vec::new());
        ret
    }
}
