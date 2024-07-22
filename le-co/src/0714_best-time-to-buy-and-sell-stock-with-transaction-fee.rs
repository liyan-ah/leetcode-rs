impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        // solled, unsold
        let mut pre = (0, -prices[0]);
        //println!("i:{}, pre:{:?}", 0, pre);
        for i in 1..prices.len() {
            // sold, fee would payed when sell
            let sold = std::cmp::max(pre.0, pre.1 + prices[i] - fee);
            let keep = std::cmp::max(pre.0 - prices[i], pre.1);
            pre.0 = sold;
            pre.1 = keep;
            //println!("i:{}, pre:{:?}", i, pre);
        }
        if pre.0 < 0 && pre.1 < 0 {
            return 0;
        }
        std::cmp::max(pre.0, pre.1)
    }
}
