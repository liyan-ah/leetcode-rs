impl Solution {
    #[allow(dead_code)]
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut dp: Vec<i32> = Vec::with_capacity(words.len());
        fn is_predecessor(words: &Vec<String>, pre: usize, cur: usize) -> bool {
            let pre_b = words[pre].as_bytes();
            let cur_b = words[cur].as_bytes();
            let mut is_decessor = false;
            let (mut p_p, mut p_c) = (0 as usize, 0 as usize);
            while p_p < pre_b.len() || p_c < cur_b.len() {
                if p_p == pre_b.len() {
                    if is_decessor == false {
                        is_decessor = true;
                        p_c += 1;
                        continue;
                    }
                    return false;
                }
                if p_c == cur_b.len() {
                    return false;
                }
                if pre_b[p_p] != cur_b[p_c] {
                    if !is_decessor {
                        is_decessor = true;
                        p_c += 1;
                        continue;
                    }
                    return false;
                }
                p_p += 1;
                p_c += 1;
            }
            true
        }
        dp.push(1);
        let mut max_chain = 1;
        let mut pos = 1 as usize;
        while pos < words.len() {
            if is_predecessor(&words, pos - 1, pos) {
                dp.push(dp[pos - 1] + 1);
                max_chain = std::cmp::max(dp[pos], max_chain);
            } else {
                dp.push(1);
            }
            pos += 1;
        }
        max_chain
    }
}
