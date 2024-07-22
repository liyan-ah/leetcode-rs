impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map_1 = std::collections::HashMap::new();
        let mut map_2 = std::collections::HashMap::new();
        let items_1: Vec<&str> = s1.split(' ').collect();
        let items_2: Vec<&str> = s2.split(' ').collect();
        for it in items_1.iter() {
            match map_1.get_mut(it) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    map_1.insert(it.clone(), 1);
                }
            }
        }
        for it in items_2.iter() {
            match map_2.get_mut(it) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    map_2.insert(it.clone(), 1);
                }
            }
        }

        let mut buddy = vec![];
        for (k, v) in map_1.iter() {
            if let Some(_) = map_2.get(k) {
                continue;
            }
            if *v >= 2 {
                continue;
            }
            buddy.push(k.clone().into());
        }
        for (k, v) in map_2.iter() {
            if let Some(_) = map_1.get(k) {
                continue;
            }
            if *v >= 2 {
                continue;
            }
            buddy.push(k.clone().into());
        }

        buddy
    }
}
