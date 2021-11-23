use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, VecDeque};

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

struct Solution {}

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        let mut current_level_sum = 0;
        while !queue.is_empty() {
            let level_cnt = queue.len();
            current_level_sum = 0;
            for _ in 0..level_cnt {
                let current_node = queue.pop_front().unwrap();
                let node = current_node.borrow();
                current_level_sum += node.val;
                if let Some(node) = &node.right {
                    queue.push_back(node.clone());
                }
                if let Some(node) = &node.left {
                    queue.push_back(node.clone());
                }
            }
        }
        current_level_sum
    }
}


fn main() {
    let mut tree1 = TreeNode::new(1);
    let mut tree2 = TreeNode::new(2);
    let mut tree3 = TreeNode::new(3);
    let mut tree4 = TreeNode::new(4);
    let mut tree5 = TreeNode::new(5);
    let mut tree6 = TreeNode::new(6);
    let mut tree7 = TreeNode::new(7);
    let mut tree8 = TreeNode::new(8);

    tree4.left = Some(Rc::new(RefCell::new(tree7)));
    tree2.left = Some(Rc::new(RefCell::new(tree4)));
    tree2.right = Some(Rc::new(RefCell::new(tree5)));
    tree6.right = Some(Rc::new(RefCell::new(tree8)));
    tree3.right = Some(Rc::new(RefCell::new(tree6)));
    tree1.right = Some(Rc::new(RefCell::new(tree3)));

    let ans = Solution::deepest_leaves_sum(Some(Rc::new(RefCell::new(tree1))));
}