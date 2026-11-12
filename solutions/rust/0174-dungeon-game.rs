impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let n = dungeon.len();
        let m = dungeon[0].len();

        let mut dp = vec![i32::MIN; m + 1];
        dp[m - 1] = 0;

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                dp[j] = dungeon[i][j] + dp[j].max(dp[j + 1]).min(0);
            }
        }

        1 - dp[0].min(0)
    }
}
