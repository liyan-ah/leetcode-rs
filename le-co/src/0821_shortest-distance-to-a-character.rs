impl Solution {
    #[allow(dead_code)]
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let s_b = s.into_bytes();
        fn find_next(s_b: &Vec<u8>, mut begin: usize, c: char) -> usize {
            if begin == s_b.len() {
                return begin;
            }
            while begin < s_b.len() {
                if char::from(s_b[begin]) == c {
                    break;
                }
                begin += 1;
            }
            return begin;
        }
        let mut pos = 0 as usize;
        let mut cur = find_next(&s_b, pos, c) as i32;
        let mut cur_next = find_next(&s_b, (cur + 1) as usize, c) as i32;
        if cur_next == (s_b.len() as i32) {
            cur_next = cur;
        }
        let mut dist: Vec<i32> = vec![];
        while pos < s_b.len() {
            let pos_i = pos as i32;
            let cur_dist = (cur - pos_i).abs();
            let next_dist = (cur_next - pos_i).abs();
            if next_dist < cur_dist {
                cur = cur_next;
                cur_next = find_next(&s_b, (cur + 1) as usize, c) as i32;
                if cur_next == s_b.len() as i32 {
                    cur_next = cur;
                }
            }
            dist.push(std::cmp::min(next_dist, cur_dist));
            pos += 1;
        }
        dist
    }
}
