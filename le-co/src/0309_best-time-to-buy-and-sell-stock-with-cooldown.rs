impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(prices.len(), vec![]);
        // own a stock; has sold last day; sold days ago
        dp[0].resize(3, 0);
        dp[0][0] = -prices[0];
        for i in 1..prices.len() {
            dp[i].resize(3, 0);
            dp[i][0] = std::cmp::max(dp[i - 1][0], dp[i - 1][2] - prices[i]);
            dp[i][1] = dp[i - 1][0] + prices[i];
            dp[i][2] = std::cmp::max(dp[i - 1][1], dp[i - 1][2]);
        }

        std::cmp::max(dp[prices.len() - 1][1], dp[prices.len() - 1][2])
    }
}
