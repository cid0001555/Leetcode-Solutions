impl Solution {
    #[allow(dead_code)]
    fn top_down(i: usize, j: usize, matrix: &[Vec<char>], memo: &mut [Vec<Option<i32>>]) -> i32 {
        if let Some(&Some(cached)) = memo.get(i).and_then(|row| row.get(j)) {
            return cached;
        }
        let result = match matrix.get(i).and_then(|row| row.get(j)) {
            Some(&'1') => {
                let down = Self::top_down(i + 1, j, matrix, memo);
                let down_right = Self::top_down(i + 1, j + 1, matrix, memo);
                let right = Self::top_down(i, j + 1, matrix, memo);
                1 + down.min(down_right).min(right)
            }
            _ => 0,
        };
        if let Some(entry) = memo.get_mut(i).and_then(|row| row.get_mut(j)) {
            *entry = Some(result);
        }
        result
    }

    #[allow(dead_code)]
    fn call_top_down(matrix: &[Vec<char>]) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut memo = vec![vec![None; n]; m];
        (0..m)
            .flat_map(|i| (0..n).map(move |j| (i, j)))
            .map(|(i, j)| Self::top_down(i, j, matrix, &mut memo))
            .max()
            .unwrap_or(0)
            .pow(2)
    }

    #[allow(dead_code)]
    fn bottom_up(matrix: &[Vec<char>]) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0_i32; n + 1]; m + 1];
        let mut max_side = 0;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let result = match matrix[i][j] {
                    '1' => {
                        let down = dp[i + 1][j];
                        let down_right = dp[i + 1][j + 1];
                        let right = dp[i][j + 1];
                        1 + down.min(down_right).min(right)
                    }
                    _ => 0,
                };
                dp[i][j] = result;
                max_side = result.max(max_side);
            }
        }
        max_side.pow(2)
    }

    #[allow(dead_code)]
    fn bottom_up_o_n_two_row(matrix: &[Vec<char>]) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let (max_side, _) = (0..m).rev().fold((0_i32, vec![0; n + 1]), |(mut max_side, mut dp), i| {
            let mut curr = vec![0; n + 1];
            for j in (0..n).rev() {
                let result = match matrix[i][j] {
                    '1' => {
                        let down = dp[j];
                        let down_right = dp[j + 1];
                        let right = curr[j + 1];
                        1 + down.min(down_right).min(right)
                    }
                    _ => 0,
                };
                curr[j] = result;
                max_side = result.max(max_side);
            }
            (max_side, curr)
        });
        max_side.pow(2)
    }

    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        // here is where you call top down
        // let top_down_result = Self::call_top_down(&matrix);

        // this is where you would call bottom up
        // let bottom_up_result = Self::bottom_up(&matrix);

        // this is where you would call 2-row bottom up
        // let two_row_result = Self::bottom_up_o_n_two_row(&matrix);

        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let (ans, _) = (0..m).rev().fold((0_i32, vec![0; n + 1]), |(mut max_side, mut prev), i| {
            let mut down_right = 0;
            for j in (0..n).rev() {
                let down = prev[j];
                let result = match matrix[i][j] {
                    '1' => {
                        let right = prev[j + 1];
                        1 + down.min(right).min(down_right)
                    }
                    _ => 0,
                };
                down_right = down;
                prev[j] = result;
                max_side = max_side.max(result);
            }
            (max_side, prev)
        });
        ans.pow(2)
    }
}
