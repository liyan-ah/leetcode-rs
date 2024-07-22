impl Solution {
    #[allow(dead_code)]
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(coins.len() + 1, vec![]);
        dp[0].resize((amount + 1) as usize, 0);
        dp[0][0] = 1;
        for i in 1..(coins.len() + 1) as usize {
            dp[i].resize((amount + 1) as usize, 0);
            for j in 0..(amount + 1) as usize {
                if coins[i - 1] > j as i32 {
                    dp[i][j] = dp[i - 1][j];
                    continue;
                }
                dp[i][j] = dp[i - 1][j] + dp[i][j - (coins[i - 1] as usize)];
            }
        }

        dp[coins.len()][amount as usize]
    }
}
