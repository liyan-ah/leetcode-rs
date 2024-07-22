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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut l = &mut dummy;

        while let Some(node) = l.next.take() {
            if node.val != val {
                l.next = Some(node);
                l = l.next.as_mut().unwrap();
            } else {
                // node.val == val
                l.next = node.next;
            }
        }

        dummy.next
    }
}
