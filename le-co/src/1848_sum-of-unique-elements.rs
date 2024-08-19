impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut uniq = std::collections::HashMap::new();
        let mut sum = 0;
        for k in nums.iter() {
            match uniq.get_mut(k) {
                Some(v) => {
                    if *v == 1 {
                        sum -= *k;
                    }
                    *v += 1;
                }
                None => {
                    uniq.insert(k.clone(), 1);
                    sum += *k;
                }
            }
        }

        sum
    }
}
