impl Solution {
    #[allow(dead_code)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s_b = s.as_bytes();
        let mut dp: Vec<bool> = vec![];
        dp.resize(s_b.len() + 1, false);
        dp[0] = true;
        let mut i = 1 as usize;
        while i < s_b.len() + 1 {
            let mut j = 0 as usize;
            while j < i {
                let sub_str = String::from_utf8(s_b[j..i].to_vec()).unwrap();
                if word_dict.contains(&sub_str) && dp[j] {
                    dp[i] = true;
                    break;
                }
                j += 1;
            }
            i += 1;
        }
        dp[s_b.len()]
    }
}
