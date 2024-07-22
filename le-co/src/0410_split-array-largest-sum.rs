impl Solution {
    pub fn check(nums: &Vec<i32>, x: i32, m: i32) -> bool {
        let (mut sum, mut cnt) = (0, 1);
        for i in 0..nums.len() {
            if sum + nums[i] > x {
                cnt += 1;
                sum = nums[i];
            } else {
                sum += nums[i];
            }
        }
        cnt <= m
    }

    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let (mut left, mut right) = (0, 0);
        for i in 0..nums.len() {
            right += nums[i];
            if left < nums[i] {
                left = nums[i];
            }
        }
        while left < right {
            let mid = left + (right - left) / 2;
            if Solution::check(&nums, mid, m) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
