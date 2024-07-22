impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

    }
}impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut pre_min, mut max) = (prices[0], 0);
        for i in 1..prices.len() {
            if prices[i] < pre_min {
                pre_min = prices[i];
                continue;
            }
            let profit = prices[i] - pre_min;
            max = std::cmp::max(max, profit);
        }

        max
    }
}
