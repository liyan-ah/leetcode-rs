impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (s_b, t_b) = (s.as_bytes(), t.as_bytes());
        let (mut s_match, mut t_i) = (0 as usize, 0 as usize);
        while t_i < t_b.len(){
            if s_match == s_b.len(){
                break;
            }
            if s_b[s_match] == t_b[t_i]{
                s_match += 1;
            }
            t_i += 1;
        }
        if s_match == s_b.len(){
            return true;
        }
        false
    }
}
