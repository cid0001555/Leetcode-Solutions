use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

const K: u64 = 0x517cc1b727220a95;

#[derive(Default)]
struct FxHasher {
    state: u64,
}

impl Hasher for FxHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.state
    }

    #[inline(always)]
    fn write(&mut self, _: &[u8]) {
        unreachable!();
    }

    #[inline(always)]
    fn write_i64(&mut self, i: i64) {
        self.state = (i as u64).wrapping_mul(K);
    }
}

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        if index_diff <= 0 || value_diff < 0 {
            return false;
        }

        let k = index_diff as usize;
        let t = value_diff as i64;
        let width = t + 1;

        let mut buckets = FxHashMap::with_capacity_and_hasher(k + 1, BuildHasherDefault::default());

        const OFFSET: i64 = i32::MIN as i64;

        for (i, &num) in nums.iter().enumerate() {
            let num_i64 = num as i64;
            let remapped_num = num_i64.wrapping_sub(OFFSET);
            let bucket_id = remapped_num / width;

            if buckets.contains_key(&bucket_id) {
                return true;
            }
            if let Some(&val) = buckets.get(&(bucket_id - 1)) {
                if num_i64 - (val as i64) <= t {
                    return true;
                }
            }
            if let Some(&val) = buckets.get(&(bucket_id + 1)) {
                if (val as i64) - num_i64 <= t {
                    return true;
                }
            }

            buckets.insert(bucket_id, num);

            if i >= k {
                let old_num = nums[i - k];
                let old_remapped = (old_num as i64).wrapping_sub(OFFSET);
                let old_bucket_id = old_remapped / width;

                if let Some(&val) = buckets.get(&old_bucket_id) {
                    if val == old_num {
                        buckets.remove(&old_bucket_id);
                    }
                }
            }
        }

        false
    }
}
