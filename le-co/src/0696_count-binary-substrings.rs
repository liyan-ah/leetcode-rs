
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s_chars:Vec<char> = s.chars().collect();
        let mut re:i32 = 0;
        let mut pre:i32 = 0;
        let mut cur:i32 = 1;
        for i in 1..s_chars.len(){
            if s_chars[i] == s_chars[i-1]{
                cur += 1;
                continue;
            }
            re += std::cmp::min(pre, cur);
            pre = cur;
            cur = 1;
        }
        return re + std::cmp::min(pre, cur);
    }
}
