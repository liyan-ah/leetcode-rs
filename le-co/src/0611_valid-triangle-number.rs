impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut num = 0 as i32;
        let mut nums = nums;
        nums.sort();
        for start in 0..nums.len() {
            for left in start + 1..nums.len() {
                for right in left + 1..nums.len() {
                    if nums[start] + nums[left] > nums[right] {
                        if nums[right] - nums[start] < nums[left] {
                            if nums[right] - nums[left] < nums[start] {
                                num += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        num
    }
}
