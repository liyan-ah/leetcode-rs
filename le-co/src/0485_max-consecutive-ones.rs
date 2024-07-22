impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut round = 0;
        for num in nums.iter() {
            if *num == 1 {
                round += 1;
                max = if round > max { round } else { max };
            } else {
                round = 0;
            }
        }

        max
    }
}
