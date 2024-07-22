
impl Solution {
    #[allow(dead_code)]
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut fast, mut slow) = (nums[0] as usize, nums[0] as usize);
            fast = nums[fast] as usize;
            fast = nums[fast] as usize;
            slow = nums[slow] as usize;
        while fast != slow{
            fast = nums[fast] as usize;
            fast = nums[fast] as usize;
            slow = nums[slow] as usize;
        }
        slow = nums[0] as usize;
        while fast != slow{
            fast = nums[fast] as usize;
            slow = nums[slow] as usize;
        }
        return fast as i32;
    }
}
