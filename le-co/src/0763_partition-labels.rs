impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut part: Vec<i32> = vec![];
        let mut pos = std::collections::HashMap::new();
        let s_b = s.into_bytes();
        // build pos map
        let mut i = 0 as usize;
        while i < s_b.len() {
            pos.insert(s_b[i], i);
            i += 1;
        }
        let mut start: usize = 0;
        let mut end: usize = 0;
        i = 0;
        while i < s_b.len() {
            end = std::cmp::max(end, *pos.get(&s_b[i]).unwrap());
            // come to the end
            if end == i {
                // push this sentence into vec
                part.push((end - start + 1) as i32);
                i += 1;
                start = i;
                end = i;
                continue;
            }
            i += 1;
        }

        part
    }
}
