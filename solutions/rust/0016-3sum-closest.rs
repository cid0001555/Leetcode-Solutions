// Problem: 0016. 3Sum Closest
// URL: https://leetcode.com/problems/3sum-closest/
// Difficulty: Medium
// Date: 2026-10-30
// Language: Rust
// Runtime: 2 ms (88.21%) | Memory: 2.3 MB (25.64%)
// Solver: LeetCode Community Scraper

impl Solution {
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let nums = nums;

    let n = nums.len();
    let mut closest = nums[0] + nums[1] + nums[2];
    let mut closest_diff = closest - target;

    for i in 0..n - 2 {
        let mut p1 = i + 1;
        let mut p2 = n - 1;

        if nums[i] + nums[i+1] + nums[i+2] > closest && closest_diff > 0 {
            break;
        }

        if nums[i] + nums[p2] + nums[p2-1] < closest && closest_diff < 0 {
            continue;
        }

        while p1 < p2 {
            let sum = nums[i] + nums[p1] + nums[p2];

            if (sum - target).abs() < closest_diff.abs() {
                closest = sum;
                closest_diff = closest - target;
            }
            if sum < target {
                p1 += 1;
            } else if sum > target {
                p2 -= 1;
            } else {
                return target;
            }
        }
    }
    closest
}
}
