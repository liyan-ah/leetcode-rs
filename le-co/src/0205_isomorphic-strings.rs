impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        if s.len() == 0 {
            return true;
        }
        let (s_c, t_c) = (s.as_bytes(), t.as_bytes());
        let mut map: HashMap<u8, u8> = HashMap::with_capacity(127); // map[s] = t
        let mut re_map: HashMap<u8, u8> = HashMap::with_capacity(127); // map[t] = s
        for i in 0..s.len() {
            if let Some(v) = map.get(&s_c[i]) {
                if v.ne(&t_c[i]) {
                    return false;
                }
            } else {
                if let Some(_) = re_map.get(&t_c[i]) {
                    return false;
                }
            }
            map.insert(s_c[i].clone(), t_c[i].clone());
            re_map.insert(t_c[i].clone(), s_c[i].clone());
        }
        true
    }
}
