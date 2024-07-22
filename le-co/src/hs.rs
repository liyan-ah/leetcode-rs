use std::collections::{hash_set, HashMap};

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

struct Solution {}

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut ty = std::collections::HashSet::new();
        for v in candy_type.iter() {
            if ty.len() >= candy_type.len() / 2 {
                break;
            }
            ty.insert(v);
        }

        ty.len() as i32
    }
}

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut feat = std::collections::HashMap::new();
        for c in license_plate.chars().into_iter() {
            if c.is_ascii_alphabetic() {
                match feat.get_mut(&c.to_ascii_lowercase()) {
                    Some(v) => {
                        *v += 1;
                    }
                    None => {
                        feat.insert(c.to_ascii_lowercase().clone(), 1);
                    }
                }
            }
        }
        let mut candidate = "".to_string();
        for word in words.iter() {
            let mut w_feat = feat.clone();
            for c in word.chars().into_iter() {
                if w_feat.len() == 0 {
                    if candidate.len() == 0 || word.len() < candidate.len() {
                        candidate = word.to_owned();
                    }
                    break;
                }
                if !c.is_ascii_alphabetic() {
                    continue;
                }
                if let Some(v) = w_feat.get_mut(&c.to_ascii_lowercase()) {
                    *v -= 1;
                    if *v == 0 {
                        w_feat.remove(&c.to_ascii_lowercase());
                    }
                }
            }
            if w_feat.len() == 0 {
                if candidate.len() == 0 || word.len() < candidate.len() {
                    candidate = word.to_owned();
                }
            }
        }
        candidate
    }
}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 && trust.len() == 0 {
            return 1;
        }
        let mut mark = std::collections::HashMap::with_capacity(n as usize);
        for it in trust.iter() {
            let [ai, bi, ..] = it[..] else { todo!() };
            match mark.get_mut(&bi) {
                Some(v) => {
                    if *v != -1 {
                        *v += 1;
                    }
                }
                None => {
                    mark.insert(bi, 1);
                }
            }
            mark.insert(ai, -1);
        }
        let mut judge = -1;
        for (&k, &v) in mark.iter() {
            if v == n - 1 {
                judge = k;
            }
        }
        judge
    }
}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut note_dic = std::collections::HashMap::new();
        let mut mag_dic = std::collections::HashMap::new();

        for c in ransom_note.as_bytes().iter() {
            match note_dic.get_mut(c) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    note_dic.insert(c.clone(), 1);
                }
            }
        }

        for c in magazine.as_bytes().iter() {
            match mag_dic.get_mut(c) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    mag_dic.insert(c.clone(), 1);
                }
            }
        }

        for (k, v) in note_dic.iter() {
            if mag_dic.get(k).is_none() {
                return false;
            }
            if let Some(m_v) = mag_dic.get(k) {
                if m_v < v {
                    return false;
                }
            }
        }

        true
    }
}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut p_h: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        for i in 0..nums.len() {
            match p_h.get_mut(&nums[i]) {
                Some(v) => {
                    if (i - *v) <= k.try_into().unwrap() {
                        return true;
                    }
                    p_h.insert(nums[i], i);
                }
                None => {
                    p_h.insert(nums[i], i);
                }
            }
        }
        false
    }
}
