// Problem: 0026. Remove Duplicates from Sorted Array
// URL: https://leetcode.com/problems/remove-duplicates-from-sorted-array/
// Difficulty: Easy
// Date: 2026-10-30
// Language: Rust
// Runtime: 0 ms (100.00%) | Memory: 2.3 MB (86.82%)
// Solver: GitHub Mirror Scraper

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if k == 0 || nums[i] != nums[k - 1] {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}
