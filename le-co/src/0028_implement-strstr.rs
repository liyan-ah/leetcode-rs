impl Solution {
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0{
            return 0;
        }
        let hay_bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();
        let (mut hy_p, mut match_p, mut match_b) = (0 as usize, 0 as usize, -1 as i32);
        while hy_p < hay_bytes.len(){
            if match_p == needle_bytes.len(){
                return match_b as i32;
            }
            if hay_bytes[hy_p] == needle_bytes[match_p]{
                if match_p == 0{
                    // println!("match_b: {}", match_b);
                    match_b = hy_p as i32;
                }
                hy_p += 1;
                match_p += 1;
                continue;
            }
            // not same
            if match_p != 0{
                hy_p = (match_b + 1) as usize;
                match_p = 0;
                match_b = -1;
                continue;
            }
            hy_p += 1;
        }
        return if match_p == needle_bytes.len(){ match_b as i32} else {-1};
    }
}
