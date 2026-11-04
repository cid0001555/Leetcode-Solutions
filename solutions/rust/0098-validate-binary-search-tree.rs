use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>) -> bool {
        if root.is_none() {
            return true;
        }
        let root = root.as_ref().unwrap().borrow();
        if !Self::dfs(&root.left, prev) {
            return false;
        }
        if prev.is_some() && prev.unwrap() >= root.val {
            return false;
        }
        *prev = Some(root.val);
        Self::dfs(&root.right, prev)
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root, &mut None)
    }
}
