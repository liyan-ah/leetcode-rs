impl Solution {
    #[allow(dead_code)]
    pub fn backspace_compare(s: String, t: String) -> bool {
        let (mut s_pos, mut t_pos) = (0 as usize, 0 as usize);
        let (mut s_b, mut t_b) = (s.into_bytes(), t.into_bytes());
        let mut pos = 0 as usize;
        while pos < s_b.len() || pos < t_b.len() {
            if pos < s_b.len() {
                if char::from(s_b[pos]) == '#' {
                    s_pos = if s_pos == 0 { 0 } else { s_pos - 1 };
                } else {
                    s_b.swap(pos, s_pos);
                    s_pos += 1;
                }
            }
            if pos < t_b.len() {
                if char::from(t_b[pos]) == '#' {
                    t_pos = if t_pos == 0 { 0 } else { t_pos - 1 };
                } else {
                    t_b.swap(t_pos, pos);
                    t_pos += 1;
                }
            }
            pos += 1;
        }
        println!("{:?}", std::str::from_utf8(s_b.get(0..s_pos).unwrap()));
        println!("{:?}", std::str::from_utf8(t_b.get(0..t_pos).unwrap()));
        if std::str::from_utf8(s_b.get(0..s_pos).unwrap())
            == std::str::from_utf8(t_b.get(0..t_pos).unwrap())
        {
            return true;
        }

        false
    }
}
