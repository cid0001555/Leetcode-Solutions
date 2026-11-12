impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
        .bytes()
        .fold(0, |acc, b| acc * 26 + (b - b'A' + 1) as i32)
    }
}
