impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let lookup:[u8;16] = [0,8,4,12,2,10,6,14,1,9,5,13,3,11,7,15];
        let mut res: [u8; 4] = n.to_le_bytes();

        for b in &mut res {
            let lo = *b & 15;
            let hi = *b >> 4;
            *b = (lookup[lo as usize] << 4) | lookup[hi as usize];
        }
        return i32::from_be_bytes(res);
    }
}
