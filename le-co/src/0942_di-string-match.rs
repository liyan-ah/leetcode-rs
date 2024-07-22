impl Solution {
    #[allow(dead_code)]
    pub fn di_string_match(s: String) -> Vec<i32> {
        let s_b = s.into_bytes();
        let mut s_v: Vec<i32> = vec![];
        let (mut lo, mut hi) = (0 as i32, s_b.len() as i32);
        let mut pos = 0 as usize;
        while pos < s_b.len() {
            let b = s_b[pos];
            if char::from(b) == 'I' {
                s_v.push(lo);
                lo += 1;
            }
            if char::from(b) == 'D' {
                s_v.push(hi);
                hi -= 1;
            }
            pos += 1;
        }
        s_v.push(lo);

        s_v
    }
}
