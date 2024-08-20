impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let items: Vec<&str> = s.split('0').collect();
        let mut has = false;
        for i in items.iter() {
            if !i.is_empty() && has {
                return false;
            }
            has = true;
        }

        true
    }
}
