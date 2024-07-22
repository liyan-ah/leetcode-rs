impl Solution {
    #[allow(dead_code)]
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut dp: Vec<i32> = vec![];
        dp.resize((n + 1) as usize, 0);
        dp[0] = 0;
        let mut near_2n = 0 as usize;
        let mut i = 1 as usize;
        while i <= n as usize {
            if i & (i - 1) == 0 {
                near_2n = i;
            }
            dp[i] = dp[i - near_2n] + 1;
            i += 1;
        }
        dp
    }
}
