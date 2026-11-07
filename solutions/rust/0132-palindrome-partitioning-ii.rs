impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let size = s.len();
        let s = s.as_bytes();
        let mut isPalin: Vec<Vec<bool>> = vec![vec![false; size]; size] ;

        for i in 0..size {
            isPalin[i][i] = true;
        }
        for len in 2..=size {
            for start in 0..=(size - len) {
                let end = start + len - 1;
                if s[start] == s[end] {
                    if len == 2 {
                        isPalin[start][end] = true;
                    }
                    else {
                        isPalin[start][end] = isPalin[start + 1][end - 1];
                    }
                }
            }
        }

        let mut cuts: Vec<i32> = vec![0; size];
        for end in 0..size {
            if isPalin[0][end] {
                cuts[end] = 0; // No cut needed if s[0..i] is a palindrome
            }
            else {
                cuts[end] = end as i32; // Max cuts possible (cut before every character)
                for start in 1..=end {
                    if isPalin[start][end] {
                        cuts[end] = cuts[end].min(cuts[start - 1] + 1);
                    }
                }
            }
        }

        return cuts[size - 1];
    }
}
