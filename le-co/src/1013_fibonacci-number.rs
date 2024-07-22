impl Solution {
    #[allow(dead_code)]
    pub fn fib(n: i32) -> i32 {
        let mut dp = (0 as i32, 1 as i32);
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }

        let mut i = 2;
        let mut value = 0 as i32;
        while i <= n {
            value = dp.0 + dp.1;
            dp.0 = dp.1;
            dp.1 = value;
            i += 1;
        }
        value
    }
}
