impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicate_letters(s: String) -> String {
        let s_b = s.into_bytes();
        let mut left: Vec<i32> = vec![];
        left.resize(26, 0);
        for pos in 0..s_b.len() {
            let p = s_b[pos] - b'a';
            left[p as usize] += 1;
        }
        let mut stack: Vec<u8> = vec![];
        let mut is_in: Vec<bool> = vec![];
        is_in.resize(26, false);
        for pos in 0..s_b.len() {
            let abs = (s_b[pos] - b'a') as usize;
            if !is_in[abs as usize] {
                while (stack.len() > 0) && (stack[stack.len() - 1] > s_b[pos]) {
                    let last_abs = (stack[stack.len() - 1] - b'a') as usize;
                    if left[last_abs] == 0 {
                        break;
                    }
                    stack.pop();
                    is_in[last_abs] = false;
                }
                stack.push(s_b[pos]);
                is_in[abs] = true;
            }
            left[abs] -= 1;
        }
        String::from_utf8(stack[0..stack.len()].to_vec()).unwrap()
    }
}
