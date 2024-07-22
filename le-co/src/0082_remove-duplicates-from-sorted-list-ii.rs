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
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut old_node = head.as_mut();
        let mut val_status = (old_node.as_ref().unwrap().val, 1);
        let mut new_head = Some(Box::new(ListNode::new(0)));
        let mut new_node = new_head.as_mut();
        while old_node.as_ref().unwrap().next.is_some() {
            old_node = old_node.unwrap().next.as_mut();
            let old_ref = old_node.as_ref().unwrap();
            if old_ref.val == val_status.0 {
                val_status.1 += 1;
                continue;
            }
            // not same, check count
            if val_status.1 == 1 {
                let node = Some(Box::new(ListNode::new(val_status.0)));
                let node_node = new_node.unwrap();
                node_node.next = node;
                new_node = node_node.next.as_mut();
            }
            val_status.0 = old_ref.val;
            val_status.1 = 1;
        }
        if val_status.1 == 1 {
            let node = Some(Box::new(ListNode::new(val_status.0)));
            let node_node = new_node.unwrap();
            node_node.next = node;
            new_node = node_node.next.as_mut();
        }
        return new_head.unwrap().next;
    }
}
