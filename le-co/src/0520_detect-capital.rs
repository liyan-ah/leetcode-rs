impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let w_c: Vec<char> = word.chars().collect();
        let is_start = w_c[0].is_ascii_uppercase();
        if w_c.len() <= 1 {
            return true;
        }
        let is_all = w_c[1].is_ascii_uppercase();
        if !is_start && is_all {
            return false;
        }
        for i in 2..w_c.len() {
            match (is_start, is_all) {
                (true, true) => {
                    if !w_c[i].is_ascii_uppercase() {
                        return false;
                    }
                }
                (true, false) => {
                    if w_c[i].is_ascii_uppercase() {
                        return false;
                    }
                }
                (false, true) => {
                    return false;
                }
                (false, false) => {
                    if w_c[i].is_ascii_uppercase() {
                        return false;
                    }
                }
            }
        }
        true
    }
}
