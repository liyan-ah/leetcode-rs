impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut parts: Vec<&str> = s.split_whitespace().collect();
        let mut re_order: Vec<String> = vec!["".to_string(); parts.len()];
        for part in parts.iter() {
            let mut chars: Vec<char> = part.chars().collect();
            let idx = chars[chars.len() - 1] as u8 - '1' as u8;
            chars.pop();
            re_order[idx as usize] = chars.iter().collect();
        }
        re_order.join(" ")
    }
}
