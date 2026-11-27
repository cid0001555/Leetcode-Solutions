impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let n = items.len();
        let b = budget as usize;
        let mut gains = vec![0; n];
        let mut cheapest = i32::MAX;

        for i in 0..n {
            cheapest = cheapest.min(items[i][1]);
            for j in 0..n {
                if items[j][0] % items[i][0] == 0 {
                    gains[i] += 1;
                }
            }
        }

        let mut dp = vec![0; b + 1];
        for i in 0..n {
            let price = items[i][1] as usize;
            let gain = gains[i];
            if price <= b {
                for money in (price..=b).rev() {
                    dp[money] = dp[money].max(dp[money - price] + gain);
                }
            }
        }

        let mut ans = 0;
        for money in 0..=b {
            ans = ans.max(dp[money] + (budget - money as i32) / cheapest);
        }
        ans
    }
}
