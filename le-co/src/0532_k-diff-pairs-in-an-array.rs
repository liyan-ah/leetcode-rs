impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut pair_num: i32 = 0;
        let mut mut_nums: Vec<i32> = nums.clone();
        mut_nums.sort();
        let mut right = mut_nums.len() - 1;
        while right > 0 {
            let mut left = 0;
            if mut_nums[right] - mut_nums[left] < k{
                break;
            }
            while left < right {
                if (mut_nums[right] - mut_nums[left]) > k {
                    while left < right && mut_nums[left] == mut_nums[left+1]{
                        left += 1;
                    }
                    left += 1;
                    continue;
                }
                if mut_nums[right] - mut_nums[left] < k{
                    break;
                }
                pair_num += 1;
                break;
            }
            while right >= 1 && mut_nums[right] == mut_nums[right-1]{
                right -= 1;
                continue;
            }
            if right == 0{
                continue;
            }
            right -= 1;
        }
        return pair_num;
    }
}
