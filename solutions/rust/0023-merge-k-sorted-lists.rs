// Problem: 0023. Merge k Sorted Lists
// URL: https://leetcode.com/problems/merge-k-sorted-lists/
// Difficulty: Hard
// Date: 2026-10-30
// Language: Rust
// Runtime: 0 ms (100.00%) | Memory: 3.2 MB (55.51%)
// Solver: GitHub Mirror Scraper

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val).reverse()
    }
}
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut pq = lists
            .into_iter()
            .filter_map(|head| head)
            .collect::<BinaryHeap<_>>();
        let mut head = None;
        let mut cur = &mut head;
        while let Some(node) = pq.pop() {
            cur = &mut cur.insert(Box::new(ListNode::new(node.val))).next;
            if let Some(next) = node.next {
                pq.push(next);
            }
        }
        head
    }
}
