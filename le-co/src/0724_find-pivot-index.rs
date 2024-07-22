impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        println!("{sum}");
        let mut left_sum = 0;
        for i in 0..nums.len() {
            if (sum - nums[i]) % 2 == 0 && (sum - nums[i]) / 2 == left_sum {
                return i as _;
            }
            left_sum += nums[i];
        }
        -1
    }
}
