impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let items = s.split(' ');
        let mut cnt = 0;
        for item in items {
            if item.trim().is_empty() {
                continue;
            }
            cnt += 1;
        }

        cnt
    }
}
