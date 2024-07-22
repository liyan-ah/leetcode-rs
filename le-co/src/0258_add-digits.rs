impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        let mut sum = 0;
        'sum: loop {
            'once: loop {
                if num < 10 {
                    break 'once;
                }
                sum += num % 10;
                num = num / 10;
            }
            sum += num;
            if sum < 10 {
                break 'sum;
            }
            num = sum;
            sum = 0;
        }
        sum
    }
}
