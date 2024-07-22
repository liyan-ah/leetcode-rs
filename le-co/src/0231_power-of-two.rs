impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        while n > 1 {
            if n % 2 != 0 {
                return false;
            }
            n = n / 2;
        }
        return if n == 1 { true } else { false };
    }
}
