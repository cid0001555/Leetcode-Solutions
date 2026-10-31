impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn backtrack(
            row: i32,
            n: i32,
            cols: &mut Vec<bool>,
            diag1: &mut Vec<bool>,
            diag2: &mut Vec<bool>,
        ) -> i32 {
            if row == n {
                return 1; // Found a valid placement
            }

            let mut count = 0;
            for col in 0..n {
                let d1 = (row + col) as usize;
                let d2 = (row - col + n) as usize; // shift to avoid negative index
                if !cols[col as usize] && !diag1[d1] && !diag2[d2] {
                    // Place queen
                    cols[col as usize] = true;
                    diag1[d1] = true;
                    diag2[d2] = true;

                    count += backtrack(row + 1, n, cols, diag1, diag2);

                    // Backtrack
                    cols[col as usize] = false;
                    diag1[d1] = false;
                    diag2[d2] = false;
                }
            }
            count
        }

        let n = n as usize;
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n];
        let mut diag2 = vec![false; 2 * n];

        backtrack(0, n as i32, &mut cols, &mut diag1, &mut diag2)
    }
}
