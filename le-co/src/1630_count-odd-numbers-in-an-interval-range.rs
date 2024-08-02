impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let num = (high - low + 1);
        if num % 2 == 0 {
            return num / 2;
        }
        return if low % 2 == 0 { num / 2 } else { num / 2 + 1 };
    }
}
