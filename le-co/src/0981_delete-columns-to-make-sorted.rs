impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut del = 0;
        if strs.len() == 0 {
            return del;
        }
        let mut char_len = strs[0].len();
        'char: for i in 0..strs[0].len() {
            let mut last = strs[0].as_bytes()[i];
            'str: for j in 1..strs.len() {
                if last > strs[j].as_bytes()[i] {
                    del += 1;
                    break 'str;
                }
                last = strs[j].as_bytes()[i];
            }
        }
        del
    }
}
