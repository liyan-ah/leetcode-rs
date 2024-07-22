impl Solution {
    #[allow(dead_code)]
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut new: Vec<i32> = nums.clone();
        let (mut less, mut great) = (0 as usize, 0 as usize);
        let mut pos = 0 as usize;
        while pos < nums.len() {
            if nums[pos] > pivot {
                great += 1;
            }
            if nums[pos] < pivot {
                less += 1;
            }
            pos += 1;
        }
        let (mut l_p, mut e_p, mut g_p) = (0 as usize, less, nums.len() - great);
        pos = 0;
        while pos < nums.len() {
            if nums[pos] == pivot {
                new[e_p] = nums[pos];
                e_p += 1;
            }
            if nums[pos] < pivot {
                new[l_p] = nums[pos];
                l_p += 1;
            }
            if nums[pos] > pivot {
                new[g_p] = nums[pos];
                g_p += 1;
            }
            pos += 1;
        }
        new
    }
}
