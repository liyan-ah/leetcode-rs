impl Solution {
    #[allow(dead_code)]
    pub fn is_match(s: String, p: String) -> bool {
        let (s_byt, p_byt) = (s.as_bytes(), p.as_bytes());
        let sing = b'?';
        let mult = b'*';
        let mut dp: Vec<Vec<bool>> = vec![];
        dp.resize(s_byt.len() + 1, vec![]);
        // init 0;
        let mut cap: Vec<bool> = vec![];
        cap.resize(p_byt.len() + 1, false);
        dp[0] = cap;
        dp[0][0] = true;
        for i in 1..(p_byt.len() + 1) {
            if p_byt[i - 1] == mult {
                dp[0][i] = true;
                continue;
            }
            break;
        }
        for i in 1..(s_byt.len() + 1) {
            let mut cap: Vec<bool> = vec![];
            cap.resize(p_byt.len() + 1, false);
            dp[i] = cap;
            for j in 1..(p_byt.len() + 1) {
                if s_byt[i - 1] == p_byt[j - 1] || p_byt[j - 1] == sing {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p_byt[j - 1] == mult {
                    dp[i][j] = dp[i - 1][j] | dp[i][j - 1];
                }
            }
        }
        dp[s_byt.len()][p_byt.len()]
    }
}
