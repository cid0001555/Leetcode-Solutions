impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        let n = {
            let mut cur = &head;
            let mut res = 0;
            while cur.is_some() {
                cur = &cur.as_ref().unwrap().next;
                res += 1;
            }
            res
        };
        k = k % n;
        if k == 0 {
            return head;
        }

        let mut cur = &mut head;
        for _ in 0..n - k - 1 {
            cur = &mut cur.as_mut().unwrap().next;
        }
        let mut res = cur.as_mut().unwrap().next.take();
        cur = &mut res;
        while cur.is_some() {
            cur = &mut cur.as_mut().unwrap().next;
        }
        *cur = head.take();
        res
    }
}
