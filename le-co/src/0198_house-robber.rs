impl Solution {
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return std::cmp::max(nums[0], nums[1]);
        }

        // include, none include
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(nums.len(), vec![]);
        // include, none include
        dp[0].resize(2, 0);
        dp[0][0] = nums[0];
        dp[1].resize(2, 0);
        dp[1][0] = nums[1];
        dp[1][1] = dp[0][0];
        for i in 2..nums.len() {
            dp[i].resize(2, 0);
            dp[i][0] = std::cmp::max(dp[i - 2][0], dp[i - 1][1]) + nums[i];
            dp[i][1] = std::cmp::max(dp[i - 1][0], dp[i - 1][1]);
        }
        std::cmp::max(dp[nums.len() - 1][0], dp[nums.len() - 1][1])
    }
}
