impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut new = vec![];
        let mut i = 0 as usize;
        while i < n as usize {
            new.push(nums[i]);
            new.push(nums[i + n as usize]);
            i += 1;
        }
        new
    }
}
