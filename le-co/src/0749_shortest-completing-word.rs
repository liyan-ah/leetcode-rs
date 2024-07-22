impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut feat = std::collections::HashMap::new();
        for c in license_plate.chars().into_iter() {
            if c.is_ascii_alphabetic() {
                match feat.get_mut(&c.to_ascii_lowercase()) {
                    Some(v) => {
                        *v += 1;
                    }
                    None => {
                        feat.insert(c.to_ascii_lowercase().clone(), 1);
                    }
                }
            }
        }
        let mut candidate = "".to_string();
        for word in words.iter() {
            let mut w_feat = feat.clone();
            for c in word.chars().into_iter() {
                if w_feat.len() == 0 {
                    if candidate.len() == 0 || word.len() < candidate.len() {
                        candidate = word.to_owned();
                    }
                    break;
                }
                if !c.is_ascii_alphabetic() {
                    continue;
                }
                if let Some(v) = w_feat.get_mut(&c.to_ascii_lowercase()) {
                    *v -= 1;
                    if *v == 0 {
                        w_feat.remove(&c.to_ascii_lowercase());
                    }
                }
            }
            if w_feat.len() == 0 {
                if candidate.len() == 0 || word.len() < candidate.len() {
                    candidate = word.to_owned();
                }
            }
        }
        candidate
    }
}
