impl Solution {
    #[allow(dead_code)]
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let (mut pre, mut max) = ((nums[0], nums[0]), nums[0].abs());
        for i in 1..nums.len() {
            pre.0 = std::cmp::min(pre.0 + nums[i], nums[i]);
            pre.1 = std::cmp::max(pre.1 + nums[i], nums[i]);
            let t_max = std::cmp::max(pre.0.abs(), pre.1.abs());
            max = std::cmp::max(max, t_max);
        }
        max
    }
}
