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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 0. reverse l1
        let mut new_l1 = None;
        let mut cur_l1 = l1;
        let mut l1_len = 0;
        while let Some(mut node) = cur_l1 {
            let next = node.next.take();
            node.next = new_l1;
            new_l1 = Some(node);
            cur_l1 = next;
            l1_len += 1;
        }

        // 1. reverse l2
        let mut new_l2 = None;
        let mut cur_l2 = l2;
        let mut l2_len = 0;
        while let Some(mut node) = cur_l2 {
            let next = node.next.take();
            node.next = new_l2;
            cur_l2 = next;
            new_l2 = Some(node);
            l2_len += 1;
        }

        // 2. merge
        let mut mer_h;
        let mut h;
        let min_len;
        if l1_len > l2_len {
            h = new_l1;
            mer_h = &new_l2;
            min_len = l2_len;
        } else {
            h = new_l2;
            mer_h = &new_l1;
            min_len = l1_len;
        }

        let mut prev = None;
        let mut upper = 0;
        let mut i = 0;
        while let Some(mut node) = h {
            if i < min_len {
                node.val = node.val + mer_h.as_ref().unwrap().val + upper;
                if node.val >= 10 {
                    node.val = node.val % 10;
                    upper = 1;
                } else {
                    upper = 0;
                }
                mer_h = &mer_h.as_ref().unwrap().next;
                i += 1;
            } else if upper != 0 {
                node.val = node.val + upper;
                if node.val >= 10 {
                    node.val = node.val % 10;
                    upper = 1;
                } else {
                    upper = 0;
                }
            }
            let next = node.next;
            node.next = prev;
            prev = Some(node);
            h = next;
        }

        if upper != 0 {
            let node = ListNode {
                val: upper,
                next: prev,
            };
            prev = Some(Box::new(node));
        }

        prev
    }
}
