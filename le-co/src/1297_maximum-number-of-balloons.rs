impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        use std::collections::HashMap;
        if text.len() < 7 {
            return 0;
        }
        let mut letter = HashMap::new();
        letter.insert('b', 0);
        letter.insert('a', 0);
        letter.insert('l', 0); // letter['l'] / 2
        letter.insert('o', 0); // letter['o'] / 2
        letter.insert('n', 0);

        for c in text.chars() {
            match letter.get(&c) {
                None => continue,
                Some(v) => {
                    letter.insert(c, *v + 1);
                }
            }
        }

        let mut num = *letter.get(&'b').unwrap();
        if *letter.get(&'a').unwrap() < num {
            num = *letter.get(&'a').unwrap();
        }
        if *letter.get(&'l').unwrap() / 2 < num {
            num = *letter.get(&'l').unwrap() / 2;
        }
        if *letter.get(&'o').unwrap() / 2 < num {
            num = *letter.get(&'o').unwrap() / 2;
        }
        if *letter.get(&'n').unwrap() < num {
            num = *letter.get(&'n').unwrap();
        }

        num
    }
}
