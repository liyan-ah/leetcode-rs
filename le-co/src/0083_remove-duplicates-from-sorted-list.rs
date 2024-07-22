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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head.as_ref();
        let mut last = None;
        let mut v = vec![];
        while let Some(node) = cur {
            cur = node.next.as_ref();
            if last.is_some() {
                if node.val == last.unwrap() {
                    continue;
                }
                v.push(node.val);
                last = Some(node.val);
                continue;
            }
            v.push(node.val);
            last = Some(node.val);
        }
        v.reverse();
        let mut new = None;
        for iter in v.iter() {
            let node = ListNode {
                val: *iter,
                next: new,
            };
            new = Some(Box::new(node));
        }
        new
    }
}
