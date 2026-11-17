impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;
        let inf = n as i32 + 1;

        let mut mismatch = vec![vec![0i32; n + 1]; n + 1];
        for i in 0..n {
            for j in i + 1..n {
                mismatch[i + 1][j + 1] =
                    mismatch[i][j] + (s[i] != s[j]) as i32;
            }
        }

        let mut cost = vec![vec![inf; n + 1]; n + 1];

        for d in 1..=n / 2 {
            let mut peel = vec![vec![0i32; n + 1]; 2];

            for blocks in 2..=n / d {
                let length = blocks * d;
                let parity = blocks % 2;

                // Ascending starts preserve the previous length's inner cost.
                for start in 0..=n - length {
                    let end = start + length;
                    let outer = mismatch[start + d][end]
                        - mismatch[start][end - d];
                    let inner = if blocks >= 4 {
                        peel[parity][start + d]
                    } else {
                        0
                    };
                    let changes = inner + outer;
                    peel[parity][start] = changes;
                    cost[start][end] = cost[start][end].min(changes);
                }
            }
        }

        let mut dp = vec![inf; n + 1];
        dp[0] = 0;

        for pieces in 1..=k {
            let mut next = vec![inf; n + 1];
            let max_end = n - 2 * (k - pieces);

            for end in 2 * pieces..=max_end {
                for start in 2 * (pieces - 1)..=end - 2 {
                    next[end] = next[end].min(dp[start] + cost[start][end]);
                }
            }

            dp = next;
        }

        dp[n]
    }
}
