impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max_len = 1;
        let s_c: Vec<char> = s.chars().collect();
        let mut last = s_c[0];
        let mut t_len = 1;
        let mut i = 1;
        while i < s_c.len() {
            if last == s_c[i] {
                t_len += 1;
            } else {
                max_len = if t_len > max_len { t_len } else { max_len };
                last = s_c[i];
                t_len = 1;
            }
            i += 1;
        }
        max_len = if t_len > max_len { t_len } else { max_len };

        max_len
    }
}
