impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut pre, mut max) = ((nums[0], nums[0]), nums[0]);
        for i in 1..nums.len() {
            let (a, b) = (pre.0 * nums[i], pre.1 * nums[i]);
            pre.0 = std::cmp::min(std::cmp::min(a, b), nums[i]);
            pre.1 = std::cmp::max(std::cmp::max(a, b), nums[i]);
            let t_max = std::cmp::max(pre.0, pre.1);
            max = std::cmp::max(t_max, max);
        }
        max
    }
}
