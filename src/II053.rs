// 这是我自己想的思路，做了真正的递归中序遍历，时间和空间常数都非常不好
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
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if root == p {
                let node = root.unwrap();
                let mut node_ref = node.borrow_mut();
                if node_ref.right.is_some() {
                    let mut node_opt = node_ref.right.take();
                    while node_opt.as_ref().unwrap().borrow().left.is_some() {
                        node_opt = node_opt.unwrap().borrow_mut().left.take();
                    }
                    return node_opt;
                }
                return p;
            }
            if let Some(node) = root {
                let mut node_ref = node.borrow_mut();
                let ret = dfs(node_ref.left.take(), p.clone());
                if ret == p {
                    drop(node_ref);
                    return Some(node);
                } else if ret.is_some() {
                    return ret;
                }
                dfs(node_ref.right.take(), p)
            } else {
                None
            }
        }
        let ret = dfs(root, p.clone());
        if ret == p {
            None
        } else {
            ret
        }
    }
}
