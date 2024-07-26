impl Solution {
    pub fn is_even(num: i32) -> bool {
        if num < 10 {
            return false;
        }
        if num < 100 {
            return true;
        }
        if num < 1000 {
            return false;
        }
        if num < 10000 {
            return true;
        }
        if num < 100000 {
            return false;
        }
        true
    }

    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for num in nums.iter() {
            if Self::is_even(*num) {
                count += 1;
            }
        }
        count
    }
}
