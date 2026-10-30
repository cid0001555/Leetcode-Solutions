// Problem: 0022. Generate Parentheses
// URL: https://leetcode.com/problems/generate-parentheses/
// Difficulty: Medium
// Date: 2026-10-30
// Language: Rust
// Runtime: 1 ms (8.24%) | Memory: 2.5 MB (12.50%)
// Solver: GitHub Mirror Scraper

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();

        fn dfs(ans: &mut Vec<String>, l: i32, r: i32, t: String, n: i32) {
            if l > n || r > n || l < r {
                return;
            }
            if l == n && r == n {
                ans.push(t);
                return;
            }
            dfs(ans, l + 1, r, format!("{}(", t), n);
            dfs(ans, l, r + 1, format!("{})", t), n);
        }

        dfs(&mut ans, 0, 0, String::new(), n);
        ans
    }
}
