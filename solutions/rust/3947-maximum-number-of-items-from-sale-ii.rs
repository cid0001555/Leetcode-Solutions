impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let n = items.len();
        let mut frequency = vec![0usize; n + 1];
        let mut min_price = i64::MAX;

        for item in &items {
            frequency[item[0] as usize] += 1;
            min_price = min_price.min(item[1] as i64);
        }

        let mut free_count = vec![0usize; n + 1];
        for factor in 1..=n {
            if frequency[factor] == 0 {
                continue;
            }
            let mut count = 0;
            for multiple in (factor..=n).step_by(factor) {
                count += frequency[multiple];
            }
            free_count[factor] = count - 1;
        }

        let mut deals = Vec::with_capacity(n);
        for item in &items {
            let price = item[1] as i64;
            let count = free_count[item[0] as usize];
            if count > 0 && price < 2 * min_price && price <= budget as i64 {
                deals.push((price, count as i64));
            }
        }
        deals.sort_unstable_by_key(|deal| deal.0);

        let mut remaining = budget as i64;
        let mut answer = 0i64;

        for (price, count) in deals {
            if price > remaining {
                break;
            }
            let take = count.min(remaining / price);
            answer += 2 * take;
            remaining -= take * price;
        }

        (answer + remaining / min_price) as i32
    }
}
