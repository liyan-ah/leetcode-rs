impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0 as usize, numbers.len()-1);
        while left < right{
            let sum = numbers[left] + numbers[right];
            if sum > target{
                right -= 1;
                continue;
            }
            if sum < target{
                left += 1;
                continue;
            }
            return vec![(left+1) as i32, (right+1) as i32];
        }
        return vec![];
    }
}
