impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut pre: Vec<(usize, usize)> = Vec::new();
        let mut i = 0;

        while i < words.len() {
            let mut cnt = 0; let mut spaces = 0; let mut wds = 0; let mut j = i;
            while j < words.len() && cnt < max_width {
                let curr = words[j].len() + 1;
                if curr + cnt <= max_width {
                    spaces += 1; wds += 1; cnt += curr; j += 1;
                } else if curr + cnt - 1 == max_width {
                    cnt += curr - 1; wds += 1; j += 1; break;
                } else {
                    break;
                }
            }
            spaces += max_width - cnt;
            pre.push((spaces, wds));
            i = j;
        }

        let mut ans: Vec<String> = Vec::new();
        let mut prev = -1i32;

        for k in 0..pre.len() {
            let (total_spaces, wds) = pre[k];
            let gaps = if wds > 1 { wds - 1 } else { 1 };
            let sp = total_spaces / gaps;
            let mut rem = total_spaces % gaps;

            if k == pre.len() - 1 {
                let mut s = String::new();
                for j in (prev + 1) as usize..(prev + 1 + wds as i32) as usize {
                    s.push_str(&words[j]);
                    if j < (prev + wds as i32) as usize { s.push(' '); }
                }
                while s.len() < max_width { s.push(' '); }
                ans.push(s); break;
            }

            if wds == 1 {
                let mut s = words[(prev + 1) as usize].clone();
                s.push_str(&" ".repeat(total_spaces));
                ans.push(s); prev += 1; continue;
            }

            let mut s = String::new();
            for j in (prev + 1) as usize..(prev + wds as i32) as usize {
                s.push_str(&words[j]);
                if s.len() == max_width { continue; }
                let mut spaces_to_add = sp;
                if rem > 0 { spaces_to_add += 1; rem -= 1; }
                s.push_str(&" ".repeat(spaces_to_add));
            }
            let last_idx = (prev + wds as i32) as usize;
            if last_idx < words.len() { s.push_str(&words[last_idx]); }
            ans.push(s);
            prev += wds as i32;
        }
        ans
    }
}
