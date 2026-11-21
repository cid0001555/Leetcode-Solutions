impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let n = parent.len();
        let b = s.as_bytes();

        let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 1..n {
            children[parent[i] as usize].push(i);
        }

        let mut last = [-1i32; 26];
        let mut saved = vec![-1i32; n];
        let mut size = vec![1i32; n];

        let mut stack: Vec<(usize, bool)> = vec![(0, false)];

        while let Some((x, leaving)) = stack.pop() {
            let c = (b[x] - b'a') as usize;

            if !leaving {
                saved[x] = last[c];
                last[c] = x as i32;
                stack.push((x, true));
                for &ch in &children[x] {
                    stack.push((ch, false));
                }
            } else {
                last[c] = saved[x];
                if x != 0 {
                    let new_parent = (if saved[x] != -1 { saved[x] } else { parent[x] }) as usize;
                    size[new_parent] += size[x];
                }
            }
        }
        size
    }
}
