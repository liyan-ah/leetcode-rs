impl Solution {
    pub fn sum_n(mut nums: Vec<i32>, val: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = vec![];

        let mut i = 0;
        while i < nums.len() {
            if i != 0 && nums[i] == nums[i - 1] {
                i += 1;
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            let target = val - nums[i];
            while left < right {
                if left != i + 1 && nums[left] == nums[left - 1] {
                    left += 1;
                    continue;
                }
                if right != nums.len() - 1 && nums[right] == nums[right + 1] {
                    right -= 1;
                    continue;
                }
                if nums[left] + nums[right] > target {
                    right -= 1;
                } else if nums[left] + nums[right] < target {
                    left += 1;
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                }
            }
            i += 1;
        }

        res
    }
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        Solution::sum_n(nums, 0)
    }
}
