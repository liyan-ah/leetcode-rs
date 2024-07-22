impl Solution {
    #[allow(dead_code)]
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if coins.len() == 0 || amount == 0 {
            return 0;
        }
        if coins.len() == 1 {
            return if amount % coins[0] == 0 {
                amount / coins[0]
            } else {
                -1
            };
        }
        let mut dp: Vec<i32> = vec![];
        dp.resize((amount + 1) as usize, 0);
        for i in 0..coins.len() {
            if coins[i] < amount {
                dp[(coins[i] as usize)] = 1;
            } else if coins[i] > amount {
                continue;
            } else if coins[i] == amount {
                return 1;
            }
        }
        for i in 1..amount as usize {
            if dp[i] == 0 {
                continue;
            }
            for j in 0..coins.len() {
                let n_p = i + (coins[j]) as usize;
                if n_p > amount as usize {
                    continue;
                }
                if dp[n_p] == 0 {
                    dp[n_p] = dp[i] + 1;
                } else if dp[n_p] != 0 {
                    dp[n_p] = std::cmp::min(dp[i] + 1, dp[n_p]);
                };
            }
        }
        return if dp[amount as usize] == 0 {
            -1
        } else {
            dp[amount as usize]
        };
    }
}
