impl Solution {
    #[allow(dead_code)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut dp: Vec<(bool, Vec<String>)> = vec![];
        let s_b = s.as_bytes();
        dp.resize(s_b.len() + 1, (false, vec![]));

        dp[0] = (true, vec![String::from("")]);
        let mut i = 1;
        while i <= s_b.len() {
            let mut j = 0;
            while j < i {
                let sub_str = String::from_utf8(s_b[j..i].to_vec()).unwrap();
                if dp[j].0 == true && word_dict.contains(&sub_str) {
                    dp[i].0 = true;
                    let mut v: Vec<String> = vec![];
                    for iter in &dp[j].1 {
                        let mut new_arr = iter.to_owned() + &String::from(" ") + &sub_str;
                        v.push(new_arr.trim().to_owned());
                    }
                    dp[i].1.extend(v);
                }
                j += 1;
            }
            i += 1;
        }

        dp[s_b.len()].1.to_owned()
    }
}
