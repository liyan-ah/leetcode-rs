impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_c = std::collections::HashMap::new();
        for c in s.chars().into_iter() {
            match s_c.get_mut(&c) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    s_c.insert(c, 1);
                }
            }
        }
        for c in t.chars().into_iter() {
            match s_c.get_mut(&c) {
                Some(v) => {
                    *v -= 1;
                    if *v == 0 {
                        s_c.remove(&c);
                    }
                }
                None => {
                    return false;
                }
            }
        }
        s_c.len() == 0
    }
}
