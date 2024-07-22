impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut checked, mut pos) = (1 as usize, 1 as usize);
        let mut status = (nums[0], 1i32);
        while pos < nums.len() {
            if status.0 == nums[pos] {
                status.1 += 1;
                if status.1 > 2 {
                    pos += 1;
                    continue;
                }
                nums.swap(checked, pos);
                checked += 1;
                pos += 1;
                continue;
            }
            status.0 = nums[pos];
            status.1 = 1;
            nums.swap(checked, pos);
            checked += 1;
            pos += 1;
        }
        return checked as i32;
    }
}
