/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

#[derive(PartialEq, Eq, Clone, Debug)]
struct LinkNode<T> {
    val: T,
    next: Option<Box<LinkNode<T>>>,
}

impl<T> LinkNode<T> {
    fn new(val: T) -> Self {
        LinkNode { val, next: None }
    }
}

struct MyHashSet {
    head: Option<Box<LinkNode<i32>>>,
}
impl MyHashSet {
    fn new() -> Self {
        Self { head: None }
    }

    fn add(&mut self, key: i32) {
        let mut cur = &self.head;
        while let Some(node) = cur {
            if node.val == key {
                return;
            }
            cur = &node.next
        }
        let node = LinkNode {
            val: key,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
        return;
    }

    fn remove(&mut self, key: i32) {
        let mut dummy = LinkNode {
            val: -1,
            next: self.head.take(),
        };
        let mut cur = &mut dummy;
        while let Some(mut node) = cur.next.take() {
            if node.val == key {
                cur.next = node.next.take();
                continue;
            }
            // node.val != key
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
        self.head = dummy.next;
    }

    fn contains(&self, key: i32) -> bool {
        let mut cur = &self.head;
        while let Some(node) = cur {
            if node.val == key {
                return true;
            }
            cur = &node.next;
        }
        false
    }
}
