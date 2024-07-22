impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s_b = s.as_bytes();
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(s_b.len(), vec![]);
        for i in 0..s_b.len() {
            dp[i] = vec![];
            dp[i].resize(s_b.len(), 0);
        }
        let mut i = s_b.len() - 1;
        loop {
            dp[i][i] = 1;
            let mut j = i + 1;
            while j < s_b.len() {
                if s_b[i] == s_b[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j - 1]);
                }
                j += 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        dp[0][s_b.len() - 1]
    }
}
