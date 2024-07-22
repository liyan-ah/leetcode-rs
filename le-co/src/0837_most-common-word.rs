impl Solution {
    pub fn most_common_word(mut paragraph: String, mut banned: Vec<String>) -> String {
        banned.push("!".into());
        banned.push("?".into());
        banned.push("'".into());
        banned.push(",".into());
        banned.push(";".into());
        banned.push(".".into());
        for word in banned.iter() {
            paragraph = paragraph.replace(word, " ");
        }
        let items: Vec<&str> = paragraph.split(' ').collect();
        let mut count = std::collections::HashMap::new();
        let (mut max_count, mut str_s) = (0, "".to_string());
        for iter in items.iter() {
            if iter.is_empty() {
                continue;
            }
            let v = iter.to_ascii_lowercase();
            match count.get_mut(&v) {
                Some(cnt) => {
                    *cnt += 1;
                    if max_count < *cnt {
                        str_s = v.clone();
                        max_count = *cnt;
                    }
                }
                None => {
                    if max_count < 1 {
                        str_s = v.clone();
                        max_count = 0;
                    }
                    count.insert(v, 1);
                }
            }
        }
        println!("{:?}, {}, {}", count, max_count, str_s);

        str_s
    }
}
