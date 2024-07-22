impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let mut sum = 0;
        for v in arr.iter() {
            sum += *v;
        }
        if sum % 3 != 0 {
            return false;
        }

        let mut sum_0 = 0;
        for i in 0..(arr.len() - 2) {
            sum_0 += arr[i];
            if sum_0 == sum / 3 {
                let mut sum_1 = 0;
                for j in i + 1..(arr.len() - 1) {
                    sum_1 += arr[j];
                    if sum_1 == sum / 3 {
                        return true;
                    }
                }
            }
        }

        false
    }
}
