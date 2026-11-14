impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut pre = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = pre.take();
            pre = Some(node);
        }
        pre
    }
}
