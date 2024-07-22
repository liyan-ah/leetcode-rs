impl Solution {
    #[allow(dead_code)]
    pub fn min_steps(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![];
        dp.resize((n + 1) as usize, 0);
        let mut i = 2 as usize;
        while i <= n as usize {
            dp[i] = n * n;
            let mut j = 1 as usize;
            while j * j <= i {
                if i % j == 0 {
                    dp[i] = std::cmp::min(dp[i], dp[i / j] + j as i32);
                    dp[i] = std::cmp::min(dp[i], dp[j] + (i / j) as i32);
                }
                j += 1;
            }
            i += 1;
        }
        println!("{:?}", dp);
        dp[n as usize]
    }
}
