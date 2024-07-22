impl Solution {
    #[allow(dead_code)]
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut dp_max, mut dp_min) = (vec![], vec![]);
        dp_max.resize(nums.len(), 0);
        dp_min.resize(nums.len(), 0);
        let (mut max, mut min, mut sum) = (nums[0], nums[0], nums[0]);
        dp_max[0] = nums[0];
        dp_min[0] = nums[0];
        if nums.len() == 1 {
            return nums[0];
        }
        for i in 1..nums.len() {
            dp_max[i] = std::cmp::max(dp_max[i - 1] + nums[i], nums[i]);
            dp_min[i] = std::cmp::min(dp_min[i - 1] + nums[i], nums[i]);
            max = std::cmp::max(dp_max[i], max);
            min = std::cmp::min(dp_min[i], min);
            sum += nums[i];
        }
        let may_max = sum - min;
        if may_max == 0 {
            return max;
        }
        std::cmp::max(max, may_max)
    }
}
