impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut str_1: Vec<char> = num1.chars().collect();
        let mut str_2: Vec<char> = num2.chars().collect();
        str_1.reverse();
        str_2.reverse();
        let mut sum = vec![];
        let mut i = 0;
        let mut up = 0;
        'add: loop {
            if i >= str_1.len() || i >= str_2.len() {
                break 'add;
            }
            let sum_v = str_1[i] as u8 + str_2[i] as u8 - '0' as u8 * 2 + up;
            sum.push((sum_v % 10 + '0' as u8) as char);
            up = sum_v / 10;
            i += 1;
        }
        while i < str_1.len() {
            if up > 0 {
                let sum_v = str_1[i] as u8 - '0' as u8 + up;
                sum.push((sum_v % 10 + '0' as u8) as char);
                up = sum_v / 10;
            } else {
                sum.push(str_1[i]);
            }
            i += 1;
        }
        while i < str_2.len() {
            if up > 0 {
                let sum_v = str_2[i] as u8 - '0' as u8 + up;
                sum.push((sum_v % 10 + '0' as u8) as char);
                up = sum_v / 10;
            } else {
                sum.push(str_2[i]);
            }
            i += 1;
        }
        if up > 0 {
            sum.push((up + '0' as u8) as char);
        }
        sum.reverse();

        sum.into_iter().collect()
    }
}
