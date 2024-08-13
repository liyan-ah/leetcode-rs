impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut freq = std::collections::HashMap::new();
        for c in s.chars().into_iter() {
            if let Some(v) = freq.get_mut(&c) {
                *v += 1;
            } else {
                freq.insert(c, 1);
            }
        }
        let mut cons = 0;
        for (_, v) in freq.iter() {
            if cons != 0 && cons != *v {
                return false;
            }
            if cons == 0 {
                cons = *v;
            }
        }

        true
    }
}
