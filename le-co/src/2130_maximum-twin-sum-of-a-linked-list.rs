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
    pub fn list_length(head: Option<&Box<ListNode>>) -> i32 {
        let mut length = 0;
        let mut node = head;
        while node.is_some() {
            let node_node = node.unwrap();
            node = node_node.next.as_ref();
            length += 1;
        }
        length
    }

    pub fn move_to(head: Option<&mut Box<ListNode>>, idx: i32) -> Option<&mut Box<ListNode>> {
        let mut cur = 0;
        let mut node = head;
        while cur < idx {
            if node.is_none() {
                return node;
            }
            let node_node = node.unwrap();
            node = node_node.next.as_mut();
            cur += 1;
        }
        node
    }

    pub fn reverse(head: Option<&mut Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref().is_none() {
            return None;
        }
        let mut new_node = ListNode::new(head.as_ref().unwrap().val);
        let mut node = head.unwrap().next.as_ref();
        while node.is_some() {
            let node_node = node.unwrap();
            new_node = ListNode {
                val: node_node.val,
                next: Some(Box::new(new_node)),
            };
            node = node_node.next.as_ref();
        }
        Some(Box::new(new_node))
    }
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut head = head;
        // find length
        let length = Solution::list_length(head.as_ref());
        let mid_idx = length / 2 - 1;
        // move to the middle
        let h_mid = Solution::move_to(head.as_mut(), mid_idx);
        // reverse [mid, None)
        let h_pair = Solution::reverse(h_mid);
        // check sum max
        let mut sum_max = 0 as i32;
        let mut node = head.as_ref();
        let mut pair = h_pair.as_ref();
        while pair.is_some() {
            let node_node = node.unwrap();
            let pair_node = pair.unwrap();
            sum_max = std::cmp::max(sum_max, node_node.val + pair_node.val);
            node = node_node.next.as_ref();
            pair = pair_node.next.as_ref();
        }
        sum_max
    }
}
