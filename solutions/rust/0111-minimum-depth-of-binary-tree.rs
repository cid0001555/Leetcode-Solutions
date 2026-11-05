use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let node = root.as_ref().unwrap().borrow();
        if node.left.is_none() {
            return 1 + Self::dfs(&node.right);
        }
        if node.right.is_none() {
            return 1 + Self::dfs(&node.left);
        }
        1 + Self::dfs(&node.left).min(Self::dfs(&node.right))
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root)
    }
}
