impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        for account in accounts.into_iter() {
            let v: i32 = account.iter().sum();
            if max < v {
                max = v;
            }
        }
        max
    }
}
