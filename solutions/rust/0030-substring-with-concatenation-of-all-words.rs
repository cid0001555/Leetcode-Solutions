use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

#[derive(Default)]
pub struct FxHasher {
    hash: u64,
}

const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

impl FxHasher {
    #[inline]
    fn add(&mut self, w: u64) {
        self.hash = (self.hash.rotate_left(5) ^ w).wrapping_mul(SEED);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut b = bytes;
        while b.len() >= 8 {
            self.add(u64::from_ne_bytes(b[..8].try_into().unwrap()));
            b = &b[8..];
        }
        if b.len() >= 4 {
            self.add(u32::from_ne_bytes(b[..4].try_into().unwrap()) as u64);
            b = &b[4..];
        }
        for &x in b {
            self.add(x as u64);
        }
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

type FxBuild = BuildHasherDefault<FxHasher>;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let sb = s.as_bytes();
        let n = sb.len();
        let m = words[0].len();
        let k = words.len();
        let total = m * k;
        if n < total {
            return Vec::new();
        }

        let mut id_of: HashMap<&[u8], u32, FxBuild> =
            HashMap::with_capacity_and_hasher(k * 2, FxBuild::default());
        let mut need: Vec<i32> = Vec::with_capacity(k);
        for w in &words {
            let b = w.as_bytes();
            match id_of.get(b) {
                Some(&i) => need[i as usize] += 1,
                None => {
                    let i = need.len() as u32;
                    id_of.insert(b, i);
                    need.push(1);
                }
            }
        }

        let positions = n - m + 1;
        let mut ids: Vec<u32> = vec![u32::MAX; positions];
        for i in 0..positions {
            if let Some(&id) = id_of.get(&sb[i..i + m]) {
                ids[i] = id;
            }
        }

        let mut res: Vec<i32> = Vec::new();
        let mut seen: Vec<i32> = vec![0; need.len()];

        for offset in 0..m {
            if offset + total > n {
                break;
            }
            let mut left = offset;
            let mut count = 0usize;
            let mut i = offset;
            while i + m <= n {
                let id = ids[i];
                if id == u32::MAX {
                    while left < i {
                        seen[ids[left] as usize] -= 1;
                        left += m;
                    }
                    count = 0;
                    left = i + m;
                } else {
                    let idu = id as usize;
                    seen[idu] += 1;
                    count += 1;
                    while seen[idu] > need[idu] {
                        seen[ids[left] as usize] -= 1;
                        count -= 1;
                        left += m;
                    }
                    if count == k {
                        res.push(left as i32);
                        seen[ids[left] as usize] -= 1;
                        count -= 1;
                        left += m;
                    }
                }
                i += m;
            }
            while left < i {
                seen[ids[left] as usize] -= 1;
                left += m;
            }
        }
        res
    }
}
