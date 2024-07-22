impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        let mut left = n;
        loop {
            if left == 1 {
                return true;
            }
            if left % 2 != 0 && left % 3 != 0 && left % 5 != 0 {
                return false;
            }
            if left % 2 == 0 {
                left = left / 2;
            }
            if left % 3 == 0 {
                left = left / 3;
            }
            if left % 5 == 0 {
                left = left / 5;
            }
        }
    }
}
