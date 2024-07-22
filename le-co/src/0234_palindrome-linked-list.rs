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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head.clone();
        if head.is_none(){
            return true;
        }
        let (mut fast, mut slow) = (head.clone(), head.clone());
        println!("fast: {:p}, slow: {:p}", &fast, &slow);
        // seek middle
        while fast.is_some() {
            slow = slow.unwrap().next;
            fast = fast.unwrap().next;
            if !fast.is_some() {
                break;
            }
            fast = fast.unwrap().next;
        }
        // now slow is in the middle, reverse [middle, none]
        let mut last: Option<Box<ListNode>> = None;
        while slow.is_some() {
            let mut slow_node = slow.unwrap();
            let tmp_next = slow_node.next;
            slow_node.next = last;
            last = Some(slow_node);
            slow = tmp_next;
        }
        slow = last;
        // now slow become last half list's head, compare
        // maybe slow has one more node
        while slow.is_some() && head.is_some() {
            let slow_node = slow.unwrap();
            let head_node = head.unwrap();
            if slow_node.val != head_node.val {
                return false;
            }
            slow = slow_node.next;
            head = head_node.next;
        }
        return true;
    }
}
