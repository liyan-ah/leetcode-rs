impl Solution {
    pub fn ways_to_step(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        let mut i = 1;
        'ac: loop {
            if i as i32 > n {
                break 'ac;
            }
            if i >= 1 {
                dp[i] = (dp[i] + dp[i - 1]) % 1000000007;
            }
            if i >= 2 {
                dp[i] = (dp[i] + dp[i - 2]) % 1000000007;
            }
            if i >= 3 {
                dp[i] = (dp[i] + dp[i - 3]) % 1000000007;
            }
            i += 1;
        }
        dp[i - 1]
    }
}
