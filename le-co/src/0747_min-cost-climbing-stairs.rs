impl Solution {
    #[allow(dead_code)]
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        cost.push(0);
        let mut dp: Vec<i32> = vec![];
        dp.resize(cost.len(), 0);
        dp[0] = cost[0];
        dp[1] = cost[1];
        if cost.len() <= 3 {
            return std::cmp::min(dp[0] + cost[2], dp[1]);
        }
        for i in 0..cost.len() - 2 {
            dp[i + 2] = std::cmp::min(dp[i], dp[i + 1]) + cost[i + 2];
        }
        dp[cost.len() - 1]
    }
}
