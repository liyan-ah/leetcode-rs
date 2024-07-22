impl Solution {
    #[allow(dead_code)]
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![];
        dp.resize((n + 1) as usize, 0);
        dp[0] = 0;
        if n == 0 {
            return dp[0];
        }
        dp[1] = 1;
        if n == 1 {
            return dp[1];
        }
        let mut i = 1;
        let mut max_v = std::cmp::max(dp[0], dp[1]);
        loop {
            if 2 * i + 1 > n || 2 * i > n {
                break;
            }
            dp[(2 * i) as usize] = dp[i as usize];
            dp[(2 * i + 1) as usize] = dp[i as usize] + dp[(i + 1) as usize];
            max_v = std::cmp::max(dp[2 * i as usize], max_v);
            max_v = std::cmp::max(dp[(2 * i + 1) as usize], max_v);
            i += 1;
        }
        max_v
    }
}
