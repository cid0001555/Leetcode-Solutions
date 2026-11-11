impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.iter().skip(1).fold((nums[0], 0), |(prev, best), &cur| (cur, best.max(cur - prev))).1
    }
}
