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

impl Solution {
    pub fn can_be_increasing(mut nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return true;
        }

        let mut i = 1;
        let mut removed = false;
        while i < nums.len() {
            if nums[i] > nums[i - 1] {
                i += 1;
                continue;
            }
            if removed {
                return false;
            }
            removed = true;
            if i < nums.len() - 1 {
                if nums[i - 1] < nums[i + 1] {
                    nums[i] = nums[i - 1];
                    i += 1;
                    continue;
                }
                if nums[i] < nums[i + 1] {
                    if i < 2 {
                        i += 1;
                        continue;
                    }
                    if i >= 2 && nums[i - 2] < nums[i] {
                        i += 1;
                        continue;
                    }
                    return false;
                }
                return false;
            }
            return true;
        }

        true
    }
}
