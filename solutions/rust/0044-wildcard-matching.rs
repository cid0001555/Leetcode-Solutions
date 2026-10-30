impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (sb, pb) = (s.as_bytes(), p.as_bytes());
        let (mut si, mut pi, mut matchpos, mut star) = (0, 0, 0, -1);
        while si < sb.len() {
            if pi < pb.len() && (pb[pi] == b'?' || pb[pi] == sb[si]) {
                si += 1;
                pi += 1;
            } else if pi < pb.len() && pb[pi] == b'*' {
                star = pi as i32;
                matchpos = si;
                pi += 1;
            } else if star != -1 {
                pi = (star + 1) as usize;
                matchpos += 1;
                si = matchpos;
            } else {
                return false;
            }
        }
        while pi < pb.len() && pb[pi] == b'*' {
            pi += 1;
        }
        return pi == pb.len();
    }
}
