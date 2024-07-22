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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        // find length
        let mut length = 0;
        let mut h_ref = head.as_ref();
        while h_ref.is_some() {
            length += 1;
            let h_node = h_ref.unwrap();
            h_ref = h_node.next.as_ref();
        }
        if length <= 2{
            return;
        }
        // move to mid
        let mut i = 1;
        let mut h_mut = head.as_mut();
        while i < length / 2 {
            i += 1;
            let h_node = h_mut.unwrap();
            h_mut = h_node.next.as_mut();
        }
        // mid_node.next = None, reverse [mid, length) node
        let mut h_node = h_mut.unwrap();
        let mut r_head = h_node.next.take();
        println!("r_head: {:?}", r_head);
        h_node.next = None;
        let mut r_last: Option<Box<ListNode>> = None;
        while r_head.as_ref().unwrap().next.is_some() {
            let mut left_node = r_head.unwrap();
            r_head = left_node.next;
            left_node.next = r_last;
            r_last = Some(left_node);
        }
        let mut r_node = r_head.as_mut().unwrap();
        r_node.next = r_last;
        println!("head {:?}, r_head: {:?}", head, r_head);

        // merge
        if let Some(node) = head{
            node.next = Self::merge(r_head, node.next.take());
        }
    }

    pub fn merge(mut left:Option<Box<ListNode>>, right:Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        match(left.as_mut(), right.as_ref()){
            (None, None) => None,
            (Some(_), None) => left,
            (None, Some(_)) => right,
            (Some(l), Some(_)) => {
                l.next = Self::merge(right, l.next.take());
                left
            }
        }
    }
}
