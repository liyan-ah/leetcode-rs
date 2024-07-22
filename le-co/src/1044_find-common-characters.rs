use std::collections::HashMap;

impl Solution {
    pub fn word_info(word: &String) -> HashMap<char, i32> {
        let mut w = HashMap::new();
        for c in word.chars() {
            match w.get_mut(&c) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    w.insert(c, 1);
                }
            }
        }
        w
    }
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        // use words[0] as init.
        let mut common = Self::word_info(&words[0]);

        for i in 1..words.len() {
            let w = Self::word_info(&words[i]);
            let mut n_c = HashMap::new();
            for (k, v) in w.iter() {
                match common.get_mut(k) {
                    Some(v_o) => {
                        n_c.insert(k.clone(), if *v_o > *v { *v } else { *v_o });
                    }
                    None => {}
                }
            }
            common = n_c;
        }

        let mut res = vec![];
        for (k, v) in common.iter() {
            res.append(&mut vec![String::from(k.clone()); v.clone() as _]);
        }
        res
    }
}
