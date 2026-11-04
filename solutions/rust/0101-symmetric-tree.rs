use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (root1, root2) {
                (Some(node1), Some(node2)) => {
                    let node1 = node1.borrow();
                    let node2 = node2.borrow();
                    node1.val == node2.val
                        && dfs(node1.left.clone(), node2.right.clone())
                        && dfs(node1.right.clone(), node2.left.clone())
                }
                (None, None) => true,
                _ => false,
            }
        }

        match root {
            Some(root) => dfs(root.borrow().left.clone(), root.borrow().right.clone()),
            None => true,
        }
    }
}
