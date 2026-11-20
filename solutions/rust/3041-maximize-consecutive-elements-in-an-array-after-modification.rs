impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut prev = 0;
        let mut dp = (0, 0);
        let mut result = 0;

        for num in nums {
            dp = match num - prev {
                0 => (dp.0, dp.0 + 1),
                1 => (dp.0 + 1, dp.1 + 1),
                2 => (dp.1 + 1, 1),
                _ => (1, 1),
            };

            result = result.max(dp.0).max(dp.1);
            prev = num;
        }

        result
    }
}
