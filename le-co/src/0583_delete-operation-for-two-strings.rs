impl Solution {
    #[allow(dead_code)]
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (byt_1, byt_2) = (word1.as_bytes(), word2.as_bytes());
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(byt_1.len() + 1, vec![]);
        // init dp[0]
        dp[0].resize(byt_2.len() + 1, 0);
        for i in 0..(byt_2.len() + 1) {
            dp[0][i] = i as i32;
        }
        for i in 1..(byt_1.len() + 1) {
            dp[i].resize(byt_2.len() + 1, 0);
            dp[i][0] = i as i32;
            for j in 1..(byt_2.len() + 1) {
                if byt_1[i - 1] == byt_2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                    continue;
                }
                dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i][j - 1]) + 1;
            }
        }

        dp[byt_1.len()][byt_2.len()]
    }
}
