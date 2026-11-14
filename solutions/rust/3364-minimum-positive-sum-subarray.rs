impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let l = l as usize;
        let r = r as usize;
        let mut min_sum = i32::MAX;

        for len in l..=r {
            // Calculate the initial window sum for size `len`
            let mut window_sum: i32 = nums[..len].iter().sum();
            if window_sum > 0 {
                min_sum = min_sum.min(window_sum);
            }

            // Slide the window across the rest of the array
            for i in len..nums.len() {
                window_sum += nums[i] - nums[i - len];
                if window_sum > 0 {
                    min_sum = min_sum.min(window_sum);
                }
            }
        }

        if min_sum == i32::MAX {
            -1
        } else {
            min_sum
        }
    }
}
