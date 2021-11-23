use std::{cell::RefCell, rc::Rc};

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}
struct Solution {}

impl Solution {
    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>, parent: i32, grandparent: i32) -> i32 {
        let mut ans = 0;
        if let Some(node) = node {
            if grandparent % 2 == 0 {
                ans += node.borrow().val;
            }
            ans += Solution::dfs(node.borrow().left.clone(), node.borrow().val, parent);
            ans += Solution::dfs(node.borrow().right.clone(), node.borrow().val, parent);
        }
        ans
    }

    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(root, 1, 1)
    }
}

pub fn main() {}
