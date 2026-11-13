impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let diff = (left ^ right) as u32;
        let shift = 32 - diff.leading_zeros();

        left >> shift << shift
    }
}
