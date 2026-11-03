impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        nums.iter()
            .try_fold((0, 0), |(m, s), &n| Some((m.max(n), s + n)))
            .map_or(0, |(m, s)| m * nums.len() as i32 - s)
    }
}
