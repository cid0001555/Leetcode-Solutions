use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(nums: &Vec<i32>, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if l > r {
                return None;
            }
            let mid = (l + r) / 2;
            if mid >= nums.len() {
                return None;
            }
            let mut node = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            node.borrow_mut().left = dfs(nums, l, mid - 1);
            node.borrow_mut().right = dfs(nums, mid + 1, r);
            Some(node)
        }
        dfs(&nums, 0, nums.len() - 1)
    }
}
