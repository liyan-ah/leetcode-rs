impl Solution {
    pub fn can_be_increasing(mut nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return true;
        }

        let mut i = 1;
        let mut removed = false;
        while i < nums.len() {
            if nums[i] > nums[i - 1] {
                i += 1;
                continue;
            }
            if removed {
                return false;
            }
            removed = true;
            if i < nums.len() - 1 {
                if nums[i - 1] < nums[i + 1] {
                    nums[i] = nums[i - 1];
                    i += 1;
                    continue;
                }
                if nums[i] < nums[i + 1] {
                    if i < 2 {
                        i += 1;
                        continue;
                    }
                    if i >= 2 && nums[i - 2] < nums[i] {
                        i += 1;
                        continue;
                    }
                    return false;
                }
                return false;
            }
            return true;
        }

        true
    }
}
