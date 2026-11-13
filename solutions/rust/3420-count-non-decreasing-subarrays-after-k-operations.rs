struct SegmentTree {
    n: usize,
    arr: Vec<i32>,
    tree: Vec<usize>,
}

impl SegmentTree {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let mut segment_tree = SegmentTree {
            n,
            arr: arr.clone(),
            tree: vec![0; 4 * n],
        };
        segment_tree.build(0, 0, n - 1);
        segment_tree
    }

    fn build(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = start;
        } else {
            let mid = (start + end) / 2;
            self.build(2 * node + 1, start, mid);
            self.build(2 * node + 2, mid + 1, end);
            let left_index = self.tree[2 * node + 1];
            let right_index = self.tree[2 * node + 2];
            self.tree[node] = if self.arr[left_index] > self.arr[right_index] {
                left_index
            } else {
                right_index
            };
        }
    }

    fn query(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        if r < start || l > end {
            return -1;
        }
        if l <= start && end <= r {
            return self.tree[node] as i32;
        }

        let mid = (start + end) / 2;
        let left_max_index = self.query(2 * node + 1, start, mid, l, r);
        let right_max_index = self.query(2 * node + 2, mid + 1, end, l, r);

        if left_max_index == -1 {
            return right_max_index;
        }
        if right_max_index == -1 {
            return left_max_index;
        }
        if self.arr[left_max_index as usize] > self.arr[right_max_index as usize] {
            left_max_index
        } else {
            right_max_index
        }
    }
}

struct ArrInc {
    n: usize,
    psum: Vec<i64>,
    suff: Vec<i64>,
    arr: Vec<i64>,
    sgt: SegmentTree,
}

impl ArrInc {
    fn new(v: Vec<i32>) -> Self {
        let n = v.len();
        let arr: Vec<i64> = v.iter().map(|&x| x as i64).collect();
        let mut psum = vec![0; n];
        let mut suff = vec![0; n];
        let sgt = SegmentTree::new(v.clone());

        psum[0] = arr[0];
        for i in 1..n {
            psum[i] = psum[i - 1] + arr[i];
        }

        let mut stack: Vec<usize> = Vec::new();
        for i in (0..n).rev() {
            while !stack.is_empty() && arr[stack[stack.len() - 1]] <= arr[i] {
                stack.pop();
            }

            if stack.is_empty() {
                suff[i] = arr[i] * (n - i) as i64;
            } else {
                suff[i] = suff[stack[stack.len() - 1]] + arr[i] * (stack[stack.len() - 1] - i) as i64;
            }

            stack.push(i);
        }

        ArrInc {
            n,
            psum,
            suff,
            arr,
            sgt,
        }
    }

    fn range_query(&self, l: usize, r: usize) -> i64 {
        let m = self.sgt.query(0, 0, self.n - 1, l, r) as usize;

        let total = self.suff[l] - self.suff[m] + self.arr[m] * (r - m + 1) as i64;
        let sum = self.psum[r] - if l > 0 { self.psum[l - 1] } else { 0 };
        total - sum
    }
}

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let ai = ArrInc::new(nums);

        let n = ai.n;
        let mut res = 0;
        let mut i = 0;
        let mut j = 0;

        while j < n {
            while i < j && ai.range_query(i, j) > k as i64 {
                i += 1;
            }

            res += (j - i + 1) as i64;
            j += 1;
        }

        res
    }
}
