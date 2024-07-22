impl Solution {
    #[allow(dead_code)]
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (s1_b, s2_b, s3_b) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        if s1_b.len() + s2_b.len() != s3_b.len() {
            return false;
        }
        let mut dp: Vec<Vec<bool>> = vec![];
        dp.resize(s1_b.len() + 1, vec![]);
        // init dp[i][0]
        dp[0] = vec![];
        dp[0].resize(s2_b.len() + 1, false);
        dp[0][0] = true;
        for i in 1..(s1_b.len() + 1) {
            dp[i] = vec![];
            dp[i].resize(s2_b.len() + 1, false);
            if dp[i - 1][0] && s1_b[i - 1] == s3_b[i - 1] {
                dp[i][0] = true;
            }
        }
        // init dp[0][i]
        for i in 1..(s2_b.len() + 1) {
            if dp[0][i - 1] && s2_b[i - 1] == s3_b[i - 1] {
                dp[0][i] = true;
            }
        }
        // check s1 and s2
        for i in 1..(s1_b.len() + 1) {
            for j in 1..(s2_b.len() + 1) {
                if (dp[i - 1][j] && s1_b[i - 1] == s3_b[i + j - 1])
                    || (dp[i][j - 1] && s2_b[j - 1] == s3_b[i + j - 1])
                {
                    dp[i][j] = true;
                }
            }
        }
        //Solution::print_arr(&dp);
        dp[s1_b.len()][s2_b.len()]
    }
}
