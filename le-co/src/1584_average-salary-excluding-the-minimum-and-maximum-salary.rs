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
