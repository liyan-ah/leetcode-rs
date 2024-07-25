impl Solution {
    pub fn reformat(s: String) -> String {
        let s_c: Vec<char> = s.chars().collect();
        let mut digit = vec![];
        let mut alpha = vec![];

        for c in s_c.iter() {
            if c.is_alphabetic() {
                alpha.push(c);
            } else {
                digit.push(c);
            }
        }
        if alpha.len() + 1 < digit.len() || digit.len() + 1 < alpha.len() {
            return "".to_string();
        }
        let mut res = vec![];
        let first;
        let second;
        if alpha.len() > digit.len() {
            first = alpha;
            second = digit;
        } else {
            first = digit;
            second = alpha;
        }
        let (mut i, mut j) = (0, 0);
        while i < first.len() && j < second.len() {
            res.push(first[i]);
            res.push(second[j]);
            i += 1;
            j += 1;
        }
        if i < first.len() {
            res.push(first[i]);
        }

        res.into_iter().collect()
    }
}
