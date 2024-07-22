impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (s_b, p_b) = (s.as_bytes(), p.as_bytes());
        let mut dp: Vec<Vec<bool>> = vec![];
        dp.resize(s_b.len() + 1, vec![]);
        let dot = b'.';
        let star = b'*';
        // init
        dp[0] = vec![];
        dp[0].resize(p_b.len() + 1, false);
        dp[0][0] = true;
        for i in 1..(p_b.len() + 1) {
            if p_b[i - 1] == star {
                dp[0][i] = dp[0][i - 2];
            }
        }
        for i in 1..(s_b.len() + 1) {
            dp[i] = vec![];
            dp[i].resize(p_b.len() + 1, false);
            for j in 1..(p_b.len() + 1) {
                if p_b[j - 1] == dot {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p_b[j - 1] == star {
                    if s_b[i - 1] == p_b[j - 2] || p_b[j - 2] == dot {
                        dp[i][j] = dp[i - 1][j] | dp[i][j - 2];
                    } else {
                        dp[i][j] = dp[i][j - 2];
                    }
                } else {
                    // not special character
                    if s_b[i - 1] == p_b[j - 1] {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                }
            }
        }
        dp[s_b.len()][p_b.len()]
    }
}
