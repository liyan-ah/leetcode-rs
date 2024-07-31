struct Solution {}

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

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut sum = 0;
        let (mut min, mut max) = (salary[0], salary[1]);
        for v in salary.iter() {
            if min > *v {
                min = *v;
            }
            if max < *v {
                max = *v;
            }
            sum += *v;
        }
        (sum - (min + max)) as f64 / (salary.len() - 2) as f64
    }
}
