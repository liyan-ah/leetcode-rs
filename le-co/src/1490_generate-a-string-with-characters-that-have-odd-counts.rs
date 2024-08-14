impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n % 2 == 1 {
            return vec!['a'; n as usize].iter().collect();
        }
        let mut res = vec!['a'; (n - 1) as usize];
        res.push('b');

        res.iter().collect()
    }
}
