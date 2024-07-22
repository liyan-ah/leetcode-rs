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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut new_h = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut l = &mut new_h;
        for _ in 1..left {
            l = &mut l.as_mut().unwrap().next;
        }

        let mut prev = None;
        let mut tail = l.as_mut().unwrap().next.take();
        for _ in left..(right + 1) {
            if let Some(mut node) = tail {
                tail = node.next.take();
                node.next = prev;
                prev = Some(node);
            }
        }

        l.as_mut().unwrap().next = prev;

        for _ in left..(right + 1) {
            l = &mut l.as_mut().unwrap().next;
        }
        l.as_mut().unwrap().next = tail;

        new_h.unwrap().next
    }
}
