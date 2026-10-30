// Problem: 0012. Integer to Roman
// URL: https://leetcode.com/problems/integer-to-roman/
// Difficulty: Medium
// Date: 2026-10-30
// Language: Rust
// Runtime: 3 ms (30.29%) | Memory: 2.1 MB (94.86%)
// Solver: GitHub Mirror Scraper

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let cs = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let vs = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let mut num = num;
        let mut ans = String::new();

        for (i, &v) in vs.iter().enumerate() {
            while num >= v {
                num -= v;
                ans.push_str(cs[i]);
            }
        }

        ans
    }
}
