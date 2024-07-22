impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut div_sum = 1;
        let mut div = 2;
        loop {
            if div * div > num {
                return div_sum == num;
            }
            if num % div == 0 {
                div_sum += div;
                if num / div != div {
                    div_sum += num / div;
                }
            }
            div += 1;
        }
    }
}
