impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .zip(prices[1..].iter())
            .fold(
                0,
                |acc, (buy, sell)| if buy < sell { acc + sell - buy } else { acc },
            )
    }
}
