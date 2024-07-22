impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut note_dic = std::collections::HashMap::new();
        let mut mag_dic = std::collections::HashMap::new();

        for c in ransom_note.as_bytes().iter() {
            match note_dic.get_mut(c) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    note_dic.insert(c.clone(), 1);
                }
            }
        }

        for c in magazine.as_bytes().iter() {
            match mag_dic.get_mut(c) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    mag_dic.insert(c.clone(), 1);
                }
            }
        }

        for (k, v) in note_dic.iter() {
            if mag_dic.get(k).is_none() {
                return false;
            }
            if let Some(m_v) = mag_dic.get(k) {
                if m_v < v {
                    return false;
                }
            }
        }

        true
    }
}
