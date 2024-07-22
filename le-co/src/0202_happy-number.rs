impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        if n == 1 {
            return true;
        }
        let mut value = std::collections::HashSet::new();
        value.insert(n);
        let mut round = 1000;
        let mut contains_1 = false;
        loop {
            if round == 0 {
                return false;
            }
            let mut sum = 0;
            while n >= 10 {
                let and = n % 10;
                n = n / 10;
                sum += and * and;
                if and == 1 || n == 1 {
                    contains_1 = true;
                }
            }
            sum += n * n;
            if sum == 1 {
                return true;
            }
            if value.get(&sum).is_some() {
                return if contains_1 { false } else { true };
            }
            n = sum;
            round -= 1;
            println!("{sum}, {round}, {contains_1}");
        }
    }
}
