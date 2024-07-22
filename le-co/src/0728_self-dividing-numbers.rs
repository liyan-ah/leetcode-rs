impl Solution {
    pub fn self_divided(num: i32) -> bool {
        let mut left = num;
        loop {
            if left < 10 {
                break;
            }
            let v = left % 10;
            if v == 0 || num % v != 0 {
                return false;
            }
            left = left / 10;
        }
        if left == 0 || num % left == 0 {
            true
        } else {
            false
        }
    }

    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = vec![];
        for v in left..right + 1 {
            if Self::self_divided(v) {
                res.push(v);
            }
        }
        res
    }
}
