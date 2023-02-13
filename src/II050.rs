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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        use std::collections::HashMap;
        fn get_ans(
            node_opt: Option<Rc<RefCell<TreeNode>>>,
            mut cur_sum: i64,
            hmap: &mut HashMap<i64, i32>,
            target_sum: i32,
        ) -> i32 {
            if let Some(node) = node_opt {
                let mut node_ref = node.borrow_mut();
                cur_sum += node_ref.val as i64;
                let mut ret = *hmap.get(&(cur_sum - target_sum as i64)).unwrap_or(&0);
                *hmap.entry(cur_sum).or_insert(0) += 1;
                ret += get_ans(node_ref.left.take(), cur_sum, hmap, target_sum)
                    + get_ans(node_ref.right.take(), cur_sum, hmap, target_sum);
                hmap.entry(cur_sum).and_modify(|x| *x -= 1);
                ret
            } else {
                0
            }
        }
        let mut hmap = HashMap::new();
        hmap.insert(0, 1);
        get_ans(root, 0, &mut hmap, target_sum)
    }
}
