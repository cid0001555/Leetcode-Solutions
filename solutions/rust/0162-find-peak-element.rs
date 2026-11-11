impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);

        while l < r {
            let mid = l + (r - l) / 2;

            if nums[mid] > nums[mid + 1] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        l as i32
    }
}
