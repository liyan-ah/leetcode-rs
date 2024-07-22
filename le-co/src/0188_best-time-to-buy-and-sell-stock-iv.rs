impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if k == 0 || prices.len() <= 1 {
            return 0;
        }
        let mut pre: Vec<i32> = vec![];
        let length = ((k + 1) * 2) as usize;
        pre.resize(length, 0);
        for i in 0..((k + 1) as usize) {
            pre[i * 2] = -prices[0];
        }
        let mut max_profit = 0;
        let mut tmp_pre: Vec<i32> = vec![];
        tmp_pre.resize(length, 0);
        for i in 1..prices.len() {
            for j in 1..((k + 1) as usize) {
                tmp_pre[2 * j] = std::cmp::max(pre[2 * j], pre[2 * j - 1] - prices[i]);
                tmp_pre[2 * j + 1] = std::cmp::max(pre[2 * j + 1], pre[2 * j] + prices[i]);
            }
            for j in 1..((k + 1) as usize) {
                pre[2 * j] = tmp_pre[2 * j];
                pre[2 * j + 1] = tmp_pre[2 * j + 1];
                max_profit = std::cmp::max(max_profit, pre[2 * j + 1]);
            }
        }
        max_profit
    }
}
