impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut i = 1;
        while i < nums.len() {
            if nums[i - 1] > nums[i] {
                break;
            }
            i += 1;
        }
        if i == nums.len() {
            return true;
        }
        i += 1;
        while i < nums.len() {
            if nums[i - 1] > nums[i] {
                return false;
            }
            i += 1;
        }
        return if nums[nums.len() - 1] <= nums[0] {
            true
        } else {
            false
        };
    }
}
