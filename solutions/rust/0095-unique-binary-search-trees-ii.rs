use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::dfs(1, n)
    }

    fn dfs(i: i32, j: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans = Vec::new();
        if i > j {
            ans.push(None);
            return ans;
        }
        for v in i..=j {
            let left = Self::dfs(i, v - 1);
            let right = Self::dfs(v + 1, j);
            for l in &left {
                for r in &right {
                    ans.push(Some(Rc::new(RefCell::new(TreeNode {
                        val: v,
                        left: l.clone(),
                        right: r.clone(),
                    }))));
                }
            }
        }
        ans
    }
}
