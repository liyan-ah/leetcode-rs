impl Solution {
    #[allow(dead_code)]
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let total = nums.iter().fold(0, |s, &a| s + a);
        if target.abs() > total {
            return 0;
        }
        if (total + target) % 2 == 1 {
            return 0;
        }
        let (pos, neg) = ((total + target) / 2, (total - target) / 2);
        let cap = std::cmp::min(pos, neg);

        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(nums.len() + 1, vec![]);
        dp[0].resize((cap + 1) as usize, 0);
        dp[0][0] = 1;
        for i in 1..(nums.len() + 1) as usize {
            dp[i].resize((cap + 1) as usize, 0);
            for j in 0..((cap + 1) as usize) {
                if j < nums[i - 1] as usize {
                    dp[i][j] = dp[i - 1][j];
                    continue;
                }
                dp[i][j] = dp[i - 1][j] + dp[i - 1][(j as i32 - nums[i - 1]) as usize];
            }
        }

        dp[nums.len()][cap as usize]
    }
}
