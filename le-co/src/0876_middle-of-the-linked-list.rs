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
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = match &head {
            Some(node) => node,
            None => return None,
        };
        let mut slow = match &head {
            Some(node) => node,
            None => return None,
        };
        'seek_loop: loop {
            // slow move 1 step each time
            // fast move 2 step each time
            slow = match &slow.next {
                Some(node) => &node,
                None => break 'seek_loop,
            };
            let mut step = 2;
            while step >= 1 {
                fast = match &fast.next{
                    Some(node) => &node,
                    None => break 'seek_loop,
                };
                step -= 1;
            }
            if !fast.next.is_some(){
                break 'seek_loop;
            }
        }
        return Some(slow.clone());
    }
}
