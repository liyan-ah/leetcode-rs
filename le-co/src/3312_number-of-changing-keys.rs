impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut change = 0;
        let mut last = None;
        let mut cur;
        for iter in s.as_bytes().iter() {
            if last.is_none() {
                last = Some(iter.clone());
                continue;
            }
            cur = iter.clone();
            if last
                .unwrap()
                .to_ascii_lowercase()
                .eq(&cur.to_ascii_lowercase())
            {
                continue;
            }
            last = Some(cur);
            change += 1;
        }
        change
    }
}
