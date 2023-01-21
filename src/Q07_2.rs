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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::HashMap;
        fn build(
            preorder: &Vec<i32>,
            inorder: &Vec<i32>,
            mp: &HashMap<i32, usize>,
            ith: &mut usize,
            l: usize,
            r: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if l >= r {
                return None;
            }
            let i = *mp.get(&preorder[*ith]).unwrap();
            *ith += 1;
            let mut node = TreeNode::new(inorder[i]);
            node.left = build(preorder, inorder, mp, ith, l, i);
            node.right = build(preorder, inorder, mp, ith, i + 1, r);
            return Some(Rc::new(RefCell::new(node)));
        }
        let mut mp = HashMap::new();
        for (i, v) in inorder.iter().enumerate() {
            mp.insert(*v, i);
        }
        let mut ith = 0;
        build(&preorder, &inorder, &mp, &mut ith, 0, inorder.len())
    }
}
