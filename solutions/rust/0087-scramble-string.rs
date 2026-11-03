impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        use std::collections::HashMap;
        let (mut dp, s1, s2) = (HashMap::<(&str, &str), bool>::new(), s1.as_str(), s2.as_str());
        fn helper<'a>(
            dp: &mut HashMap<(&'a str, &'a str), bool>,
            s1: &'a str,
            s2: &'a str,
        ) -> bool {
            if let Some(&res) = dp.get(&(s1, s2)) {
                return res;
            }
            if s1 == s2 {
                dp.insert((s1, s2), true);
                return true;
            }
            let n = s1.len();
            for split_pos in 1..n {
                let (s1_left, s1_right) = (&s1[..split_pos], &s1[split_pos..]);
                let (s2_left, s2_right) = (&s2[..split_pos], &s2[split_pos..]);

                if helper(dp, s1_left, s2_left) && helper(dp, s1_right, s2_right) {
                    dp.insert((s1, s2), true);
                    return true;
                }

                let s2_left_swap = &s2[..n - split_pos];
                let s2_right_swap = &s2[n - split_pos..];

                if helper(dp, s1_left, s2_right_swap) && helper(dp, s1_right, s2_left_swap) {
                    dp.insert((s1, s2), true);
                    return true;
                }
            }
            dp.insert((s1, s2), false);
            false
        }
        helper(&mut dp, s1, s2)

    }
}
