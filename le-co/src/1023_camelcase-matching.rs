impl Solution {
    #[allow(dead_code)]
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut result: Vec<bool> = vec![];
        let p_b = pattern.as_bytes();
        for query in queries.iter() {
            let q_b = query.as_bytes();
            let (mut q, mut p) = (0 as usize, 0 as usize);
            let mut res = true;
            while q < q_b.len() {
                if q_b[q] >= b'A' && q_b[q] <= b'Z' {
                    if p >= p_b.len() {
                        res = false;
                        break;
                    }
                    if q_b[q] == p_b[p] {
                        q += 1;
                        p += 1;
                        continue;
                    } else {
                        res = false;
                        break;
                    }
                }
                if p < p_b.len() && q_b[q] == p_b[p] {
                    q += 1;
                    p += 1;
                    continue;
                }
                q += 1;
            }
            if p < p_b.len() {
                res = false;
            }
            result.push(res);
        }
        result
    }
}
