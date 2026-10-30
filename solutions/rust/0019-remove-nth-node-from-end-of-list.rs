// Problem: 0019. Remove Nth Node From End of List
// URL: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
// Difficulty: Medium
// Date: 2026-10-30
// Language: Rust
// Runtime: 0 ms (100.00%) | Memory: 2.2 MB (6.87%)
// Solver: GitHub Mirror Scraper

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow = &mut dummy;
        let mut fast = &slow.clone();
        for _ in 0..=n {
            fast = &fast.as_ref().unwrap().next;
        }
        while fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
        dummy.unwrap().next
    }
}
