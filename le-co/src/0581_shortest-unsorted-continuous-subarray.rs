impl Solution {
    #[allow(dead_code)]
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let (mut ab_min, mut ab_max) = (0 as i32, 0 as i32);
        // find ab min
        let mut flag: bool = false;
        let mut i = 1 as usize;
        while i < nums.len() {
            if (nums[i-1] > nums[i]) && (!flag) {
                flag = true;
                ab_min = nums[i];
            }
            if flag {
                if nums[i] < ab_min {
                    ab_min = nums[i];
                }
            }
            i += 1;
        }
        if ! flag{
            return 0;
        }
        i = nums.len() - 2;
        flag = false;
        while i >= 0 {
            if (nums[i] > nums[i + 1]) && (!flag) {
                flag = true;
                ab_max = nums[i];
            }
            if flag {
                if nums[i] > ab_max {
                    ab_max = nums[i];
                }
            }
            if i == 0{
                break;
            }
            i -= 1;
        }
        // find the begin of ab_min
        let (mut l, mut r) = (0 as usize, nums.len() - 1);
        i = 0;
        while i < nums.len() {
            if nums[i] > ab_min {
                l = i;
                break;
            }
            i += 1;
        }
        i = nums.len() - 1;
        while i > 0 {
            if nums[i] < ab_max {
                r = i;
                break;
            }
            i -= 1;
        }
        if i == 0 {
            r = 0;
        }
        return if l >= r { 0 } else { (r - l + 1) as i32 };
    }
}
