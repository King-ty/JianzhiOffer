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

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::LinkedList;
        let mut nodes = Vec::new();
        let mut ret = Vec::new();
        let mut layer_num = 0;
        if let Some(node) = root {
            ret.push(Vec::new());
            nodes.push(LinkedList::new());
            nodes[0].push_back(node);
        }
        while nodes.get(layer_num).is_some() {
            while !nodes[layer_num].is_empty() {
                let current = if layer_num & 1 == 1 {
                    nodes[layer_num].pop_back()
                } else {
                    nodes[layer_num].pop_front()
                };
                if let Some(node) = current {
                    let mut node_mut = node.borrow_mut();
                    ret[layer_num].push(node_mut.val);
                    let children = if layer_num & 1 == 0 {
                        [node_mut.left.take(), node_mut.right.take()]
                    } else {
                        [node_mut.right.take(), node_mut.left.take()]
                    };
                    for current in children {
                        if let Some(node) = current {
                            if nodes.len() <= layer_num + 1 {
                                ret.push(Vec::new());
                                nodes.push(LinkedList::new());
                            }
                            if layer_num & 1 == 0 {
                                nodes[layer_num + 1].push_back(node);
                            } else {
                                nodes[layer_num + 1].push_front(node);
                            }
                        }
                    }
                    if let Some(node) = node_mut.right.take() {
                        if nodes.len() <= layer_num + 1 {
                            ret.push(Vec::new());
                            nodes.push(LinkedList::new());
                        }
                        nodes[layer_num + 1].push_back(node);
                    }
                }
            }
            layer_num += 1;
        }
        ret
    }
}
