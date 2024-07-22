impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut pre = (-prices[0], 0, -prices[0], 0);
        for i in 1..prices.len() {
            let b1 = std::cmp::max(pre.0, -prices[i]);
            let s1 = std::cmp::max(pre.1, pre.0 + prices[i]);
            let b2 = std::cmp::max(pre.2, pre.1 - prices[i]);
            let s2 = std::cmp::max(pre.3, pre.2 + prices[i]);
            pre = (b1, s1, b2, s2);
        }
        std::cmp::max(pre.1, pre.3)
    }
}
