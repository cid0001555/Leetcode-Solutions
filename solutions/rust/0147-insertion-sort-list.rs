impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = head;

        while let Some(mut node) = curr {
            // Find the position to insert the node.
            let mut cursor = &mut dummy;
            while let Some(ref next) = cursor.next {
                if next.val > node.val {
                    break;
                }
                // Briefly held mutable reference to advance the cursor.
                cursor = cursor.next.as_mut().unwrap();
            }

            // Splice the node into the list.
            curr = node.next.take();  // Advances to the next node.
            node.next = cursor.next.take();  // Points to the elements after the cursor.
            cursor.next = Some(node);  // Inserts the node after the cursor.
        }

        dummy.next
    }
}
