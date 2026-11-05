use std::{cell::RefCell, collections::VecDeque, rc::Rc};
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        if let Some(root_node) = root {
            let mut q = VecDeque::new();
            q.push_back(root_node);
            while !q.is_empty() {
                let mut t = Vec::new();
                for _ in 0..q.len() {
                    if let Some(node) = q.pop_front() {
                        let node_ref = node.borrow();
                        t.push(node_ref.val);
                        if let Some(ref left) = node_ref.left {
                            q.push_back(Rc::clone(left));
                        }
                        if let Some(ref right) = node_ref.right {
                            q.push_back(Rc::clone(right));
                        }
                    }
                }
                ans.push(t);
            }
        }
        ans.reverse();
        ans
    }
}
