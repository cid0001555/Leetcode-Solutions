impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        (1..)
            .map(|exp| i32::pow(5, exp))
            .take_while(|div| *div <= n)
            .map(|div| n / div)
            .sum()
    }
}
