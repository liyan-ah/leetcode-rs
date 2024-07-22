impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut pre, mut max) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            pre = std::cmp::max(pre + nums[i], nums[i]);
            max = std::cmp::max(max, pre);
        }
        max
    }
}
