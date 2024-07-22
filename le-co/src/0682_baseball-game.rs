impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut values = vec![];
        for op in operations.into_iter() {
            let v = op.as_bytes()[0];
            if v.is_ascii_digit() || v == '-' as u8 {
                values.push(op.parse::<i32>().unwrap());
                continue;
            }
            if v == 'C' as u8 {
                values.pop();
                continue;
            }
            if v == 'D' as u8 {
                if let Some(l) = values.last() {
                    values.push(l * 2);
                }
                continue;
            }
            if v == '+' as u8 {
                if values.len() < 2 {
                    continue;
                }
                if let [.., v1, v2] = values[..] {
                    values.push(v1 + v2);
                    continue;
                }
            }
        }
        let mut sum = 0;
        for v in values.iter() {
            sum += v;
        }
        sum as i32
    }
}
