impl Solution {
    #[allow(dead_code)]
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else if n == 2 {
            return 1;
        }
        let mut dp: Vec<i32> = vec![];
        dp.resize((n + 1) as usize, 0);
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 1;
        let mut i = 0 as usize;
        loop {
            dp[i + 3] = dp[i + 2] + dp[i + 1] + dp[i];
            i += 1;
            if (i + 3) > (n as usize) {
                break;
            }
        }

        dp[n as usize]
    }
}
