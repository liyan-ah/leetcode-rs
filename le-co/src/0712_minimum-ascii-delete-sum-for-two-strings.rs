impl Solution {
    #[allow(dead_code)]
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let (b1, b2) = (s1.as_bytes(), s2.as_bytes());
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(b1.len() + 1, vec![]);
        dp[0].resize(b2.len() + 1, 0);
        dp[0][0] = 0;
        // init dp[0]
        for i in 1..(b2.len() + 1) {
            dp[0][i] = dp[0][i - 1] + b2[i - 1] as i32;
        }
        // check dp
        for i in 1..(b1.len() + 1) {
            dp[i].resize(b2.len() + 1, 0);
            dp[i][0] = dp[i - 1][0] + b1[i - 1] as i32;
            for j in 1..(b2.len() + 1) {
                if b1[i - 1] == b2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                    continue;
                }
                dp[i][j] = std::cmp::min(
                    dp[i][j - 1] + (b2[j - 1] as i32),
                    dp[i - 1][j] + (b1[i - 1] as i32),
                );
            }
        }
        dp[b1.len()][b2.len()]
    }
}
