impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let area_a = (ax2 - ax1) * (ay2 - ay1);
        let area_b = (bx2 - bx1) * (by2 - by1);
        let mut overlap = 0;

        if ax1 < bx2 && ay1 < by2 && ax2 > bx1 && ay2 > by1 {
            let start_x = ax1.max(bx1);
            let start_y = ay1.max(by1);
            let end_x = ax2.min(bx2);
            let end_y = ay2.min(by2);
            overlap = (end_x - start_x) * (end_y - start_y);
        }
        area_a + area_b - overlap
    }
}
