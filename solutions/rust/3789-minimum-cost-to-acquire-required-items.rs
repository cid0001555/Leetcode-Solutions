impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let pairs = need1.min(need2) as i64;
        let mut cost = pairs * (cost_both.min(cost1 + cost2) as i64);

        let rem1 = (need1 as i64) - pairs;
        let rem2 = (need2 as i64) - pairs;

        cost += rem1 * (cost1.min(cost_both) as i64);
        cost += rem2 * (cost2.min(cost_both) as i64);

        cost
    }
}
