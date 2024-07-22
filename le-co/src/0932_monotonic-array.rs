impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return true;
        }
        let (mut before, mut sub) = (nums[1], nums[1] - nums[0]);
        for i in 2..nums.len() {
            if sub * (nums[i] - before) < 0 {
                return false;
            }
            if nums[i] - before != 0 {
                sub = nums[i] - before;
            }
            before = nums[i];
        }
        true
    }
}
