struct Solution {}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}

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
