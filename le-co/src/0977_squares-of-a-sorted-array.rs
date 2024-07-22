impl Solution {
    #[allow(dead_code)]
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut squares: Vec<i32> = vec![];
        let (mut left, mut right) = (0 as usize, nums.len() - 1);
        while left < right {
            let right_value = nums[right] * nums[right];
            let left_value = nums[left] * nums[left];
            if left_value > right_value {
                squares.push(left_value);
                left += 1;
                continue;
            }
            squares.push(right_value);
            right -= 1;
        }
        squares.push(nums[left] * nums[left]);
        squares.reverse();
        squares
    }
}
