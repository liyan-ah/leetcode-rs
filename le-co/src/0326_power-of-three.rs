impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        if n < 1 {
            return false;
        }
        while n > 1 {
            if n % 3 != 0 {
                return false;
            }
            n = n / 3;
        }

        return if n == 1 { true } else { false };
    }
}
