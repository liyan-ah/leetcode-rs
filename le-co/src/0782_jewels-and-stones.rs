impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut num = 0;
        for c in stones.chars().into_iter() {
            if jewels.contains(c) {
                num += 1;
            }
        }
        num
    }
}
