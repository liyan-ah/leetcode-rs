impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut digits: Vec<char> = number.replace(" ", "").replace("-", "").chars().collect();
        let mut final_str = vec![];
        let mut i = 0;
        while i < digits.len() {
            if digits.len() - i == 4 && i % 3 == 0 {
                final_str.push(digits[i]);
                final_str.push(digits[i + 1]);
                final_str.push('-');
                final_str.push(digits[i + 2]);
                final_str.push(digits[i + 3]);
                break;
            }
            final_str.push(digits[i]);
            i += 1;
            if i % 3 == 0 && i != digits.len() {
                final_str.push('-');
            }
        }

        final_str.iter().collect()
    }
}
