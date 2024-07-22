impl Solution {
    pub fn convert_to_base_n(num: i32, base: i32) -> String {
        let mut left = num;
        let mut v = vec![];
        loop {
            if left.abs() < base.abs() {
                break;
            }
            v.push(((left % base).abs() as u8 + '0' as u8) as char);
            left = left / base;
        }
        v.push((left.abs() as u8 + '0' as u8) as char);
        if num < 0 {
            v.push('-');
        }
        v.reverse();
        v.iter().collect()
    }
    pub fn convert_to_base7(num: i32) -> String {
        Self::convert_to_base_n(num, 7)
    }
}
