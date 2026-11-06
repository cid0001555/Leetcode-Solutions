use std::collections::HashMap;

struct Fenwick { t: Vec<i32> }

impl Fenwick {
    fn new(n: usize) -> Self { Self { t: vec![0; n + 1] } }
    fn add(&mut self, i: usize, v: i32) {
        let mut i = i + 1;
        while i < self.t.len() { self.t[i] += v; i += i & i.wrapping_neg(); }
    }
    fn prefix(&self, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 { s += self.t[i]; i &= i - 1; }
        s
    }
}

impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, mut queries: Vec<i32>) -> Vec<i32> {
        logs.sort_unstable_by_key(|l| l[1]);
        let mut qs: Vec<(usize, usize, usize)> = queries
        .iter()
        .enumerate()
        .map(|(qi, &tt)| {
            let left = logs.partition_point(|l| l[1] < tt-x);
            let right = logs.partition_point(|l| l[1] <= tt);
            (right, left, qi)
        })
        .collect();
        qs.sort_unstable();

        let mut bit = Fenwick::new(logs.len());
        let mut last: HashMap<i32, usize> = HashMap::new();
        let mut ans = vec![0; queries.len()];
        let mut done = 0;

        for (right, left, qi) in qs {
            while done < right {
                let id = logs[done][0];
                if let Some(prev) = last.insert(id, done) {
                    bit.add(prev, -1);
                }
                bit.add(done, 1);
                done += 1;
            }
            ans[qi] = n-(bit.prefix(right) - bit.prefix(left));
        }

        ans
    }
}
