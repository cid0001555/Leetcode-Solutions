use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut num: i32) -> i32 {
        if node.is_none() {
            return 0;
        }
        let node = node.as_ref().unwrap().borrow();
        num = num * 10 + node.val;
        if node.left.is_none() && node.right.is_none() {
            return num;
        }
        Self::dfs(&node.left, num) + Self::dfs(&node.right, num)
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 0)
    }
}
