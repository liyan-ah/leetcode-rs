impl Solution {
    #[allow(dead_code)]
    pub fn rob_from_to(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        if end - start <= 1 {
            return std::cmp::max(nums[start], nums[end]);
        }
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(nums.len(), vec![]);
        // include, none include
        dp[start].resize(2, 0);
        dp[start][0] = nums[start];
        dp[start + 1].resize(2, 0);
        dp[start + 1][0] = nums[start + 1];
        dp[start + 1][1] = dp[start][0];
        for i in (start + 2)..(end + 1) {
            dp[i].resize(2, 0);
            dp[i][0] = std::cmp::max(dp[i - 2][0], dp[i - 1][1]) + nums[i];
            dp[i][1] = std::cmp::max(dp[i - 1][0], dp[i - 1][1]);
        }
        std::cmp::max(dp[end][0], dp[end][1])
    }

    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return std::cmp::max(nums[0], nums[1]);
        }
        // [0, nums.len()-2]
        let max1 = Solution::rob_from_to(nums.as_ref(), 0, nums.len() - 2);
        // [1, nums.len()-1]
        let max2 = Solution::rob_from_to(nums.as_ref(), 1, nums.len() - 1);
        std::cmp::max(max1, max2)
    }
}
