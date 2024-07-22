// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut k = k;
        if head.is_none() {
            return head;
        }
        // get list length
        let mut length = 0;
        let mut node = head.as_mut();
        while node.is_some() {
            length += 1;
            let node_node = node.as_ref().unwrap();
            if node_node.next.is_none() {
                break;
            }
            node = node.unwrap().next.as_mut();
        }
        if length <= k {
            k = k % length;
        }
        if k == 0 {
            return head;
        }
        node = head.as_mut();
        let mut i = 1;
        while i < length-k{
            node = node.unwrap().next.as_mut();
            i += 1;
        }
        let mut node_node = node.unwrap();
        let mut new_head = node_node.next.clone();
        node_node.next = None;
        let mut new = new_head.as_mut();
        loop {
            let new_node = new.as_ref().unwrap();
            if new_node.next.is_none() {
                break;
            }
            new = new.unwrap().next.as_mut();
        }
        let new_node = new.unwrap();
        new_node.next = head;

        new_head
    }
}
