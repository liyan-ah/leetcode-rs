impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let (s_b, e_b) = (start.into_bytes(), end.into_bytes());
        if s_b.len() != e_b.len() {
            return false;
        }
        let (mut s_p, mut e_p) = (0 as usize, 0 as usize);
        let (mut s_x, mut e_x) = (0 as usize, 0 as usize);
        while s_p < s_b.len() || e_p < e_b.len() {
            while s_p < s_b.len() && char::from(s_b[s_p]) == 'X' {
                s_p += 1;
                s_x += 1;
            }
            while e_p < e_b.len() && char::from(e_b[e_p]) == 'X' {
                e_p += 1;
                e_x += 1;
            }
            if s_p == s_b.len() || e_p == e_b.len() {
                break;
            }
            if s_b[s_p] != e_b[e_p] {
                return false;
            }
            if char::from(s_b[s_p]) == 'R' {
                if s_p > e_p {
                    return false;
                }
            } else if char::from(s_b[s_p]) == 'L' {
                if s_p < e_p {
                    return false;
                }
            }
            s_p += 1;
            e_p += 1;
        }
        if s_x != e_x {
            return false;
        }
        true
    }
}
