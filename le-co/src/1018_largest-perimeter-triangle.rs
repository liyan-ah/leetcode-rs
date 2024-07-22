impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut max_perimter = 0;
        let mut pos = nums.len() - 1;
        while pos >= 2 {
            // right donnot need to move, why?
            let (mut left, mut right) = (0 as usize, pos - 1);
            while left < right {
                if nums[pos] - nums[left] >= nums[right] {
                    left += 1;
                    continue;
                }
                if nums[pos] - nums[right] >= nums[left] {
                    left += 1;
                    continue;
                }
                if nums[left] + nums[right] <= nums[pos] {
                    left += 1;
                    continue;
                }
                let permiter = nums[pos] + nums[left] + nums[right];
                if permiter > max_perimter {
                    max_perimter = permiter;
                }
                left += 1;
            }
            pos -= 1;
        }
        max_perimter
    }
}
