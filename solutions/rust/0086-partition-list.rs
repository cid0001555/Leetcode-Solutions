impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut l = ListNode::new(0);
        let mut r = ListNode::new(0);
        let mut tl = &mut l;
        let mut tr = &mut r;
        let mut current = head;
        while let Some(mut node) = current {
            current = node.next.take();
            if node.val < x {
                tl.next = Some(node);
                tl = tl.next.as_mut().unwrap();
            } else {
                tr.next = Some(node);
                tr = tr.next.as_mut().unwrap();
            }
        }
        tr.next = None;
        tl.next = r.next;
        l.next
    }
}
