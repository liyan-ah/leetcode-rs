impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut max_sum = 0;
        let mut pos = 0 as usize;
        while pos < nums.len() {
            max_sum += nums[pos];
            pos += 2;
        }
        return max_sum;
    }
}
