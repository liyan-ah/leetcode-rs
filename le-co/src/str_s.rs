use std::{collections::HashMap, io::BufRead};

struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
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

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut num = 0;

        for v in column_title.chars().into_iter() {
            num *= 26;
            num += v as i32 - 'A' as i32 + 1;
        }

        num
    }
}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        if s.len() == 0 {
            return true;
        }
        let (s_c, t_c) = (s.as_bytes(), t.as_bytes());
        let mut map: HashMap<u8, u8> = HashMap::with_capacity(127); // map[s] = t
        let mut re_map: HashMap<u8, u8> = HashMap::with_capacity(127); // map[t] = s
        for i in 0..s.len() {
            if let Some(v) = map.get(&s_c[i]) {
                if v.ne(&t_c[i]) {
                    return false;
                }
            } else {
                if let Some(_) = re_map.get(&t_c[i]) {
                    return false;
                }
            }
            map.insert(s_c[i].clone(), t_c[i].clone());
            re_map.insert(t_c[i].clone(), s_c[i].clone());
        }
        true
    }
}

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut left = column_number;
        let mut title = vec![];
        while left > 0 {
            println!("{left}");
            let v = (left - 1) % 26;
            let c = (v as u8 + 'A' as u8) as char;
            title.push(c);
            left = (left - 1) / 26;
        }
        title.reverse();
        title.iter().collect::<String>()
    }
}

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut fmt = vec![];
        let mut cnt = 0;
        for v in s.chars().into_iter().rev() {
            if !v.is_ascii_alphanumeric() {
                continue;
            }
            if cnt % k == 0 && cnt != 0 {
                fmt.push('-');
            }
            if v.is_ascii_alphabetic() {
                fmt.push(v.to_ascii_uppercase());
            } else {
                fmt.push(v);
            }
            cnt += 1;
        }

        fmt.into_iter().rev().collect()
    }
}

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let items = s.split(' ');
        let mut cnt = 0;
        for item in items {
            if item.trim().is_empty() {
                continue;
            }
            cnt += 1;
        }

        cnt
    }
}

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut values = vec![];
        for op in operations.into_iter() {
            let v = op.as_bytes()[0];
            if v.is_ascii_digit() || v == '-' as u8 {
                values.push(op.parse::<i32>().unwrap());
                continue;
            }
            if v == 'C' as u8 {
                values.pop();
                continue;
            }
            if v == 'D' as u8 {
                if let Some(l) = values.last() {
                    values.push(l * 2);
                }
                continue;
            }
            if v == '+' as u8 {
                if values.len() < 2 {
                    continue;
                }
                if let [.., v1, v2] = values[..] {
                    values.push(v1 + v2);
                    continue;
                }
            }
        }
        let mut sum = 0;
        for v in values.iter() {
            sum += v;
        }
        println!("{:?}", values);
        sum as i32
    }
}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut e = std::collections::HashSet::new();
        for str_s in emails.iter() {
            let items: Vec<_> = str_s.split('@').collect();
            assert_eq!(items.len(), 2);
            let mut local = items[0];
            let domain = items[1];
            let items: Vec<_> = local.split('+').collect();
            local = items[0];
            let local = local.replace(".", "");
            e.insert((local, domain));
        }

        e.len() as i32
    }
}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut rp = 0;
        let mut start = 0;
        let s_c: Vec<char> = s.chars().collect();
        for i in 1..s.len() / 2 {
            if s_c[i] == s_c[rp] {}
        }
        true
    }
}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let w_c: Vec<char> = word.chars().collect();
        let is_start = w_c[0].is_ascii_uppercase();
        if w_c.len() <= 1 {
            return true;
        }
        let is_all = w_c[1].is_ascii_uppercase();
        for i in 2..w_c.len() {
            match (is_start, is_all) {
                (true, true) => {
                    if !w_c[i].is_ascii_uppercase() {
                        return false;
                    }
                }
                (true, false) => {
                    if w_c[i].is_ascii_uppercase() {
                        return false;
                    }
                }
                (false, true) => {
                    return false;
                }
                (false, false) => {
                    if w_c[i].is_ascii_uppercase() {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_c = std::collections::HashMap::new();
        for c in s.chars().into_iter() {
            match s_c.get_mut(&c) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    s_c.insert(c, 1);
                }
            }
        }
        for c in t.chars().into_iter() {
            match s_c.get_mut(&c) {
                Some(v) => {
                    *v -= 1;
                    if *v == 0 {
                        s_c.remove(&c);
                    }
                }
                None => {
                    return false;
                }
            }
        }
        s_c.len() == 0
    }
}

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_ascii_lowercase()
    }
}

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut a_d = 0;
        let mut cons_l = 0;
        for c in s.chars().into_iter() {
            match c {
                'A' => {
                    a_d += 1;
                    if a_d >= 2 {
                        return false;
                    }
                    cons_l = 0;
                }
                'L' => {
                    cons_l += 1;
                    if cons_l >= 3 {
                        return false;
                    }
                }
                'P' => {
                    cons_l = 0;
                }
                _ => {
                    cons_l = 0;
                }
            }
        }
        true
    }
}

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut num = 0;
        for c in stones.chars().into_iter() {
            if jewels.contains(c) {
                num += 1;
            }
        }
        num
    }
}

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

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut del = 0;
        if strs.len() == 0 {
            return del;
        }
        let mut char_len = strs[0].len();
        for i in 0..strs[0].len() {
            let mut last = strs[0].as_bytes()[i];
            'str: for j in 1..strs.len() {
                if last > strs[j].as_bytes()[i] {
                    del += 1;
                    break 'str;
                }
                last = strs[j].as_bytes()[i];
            }
        }
        del
    }
}

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

impl Solution {
    // [l, r)
    pub fn find_between(letters: &Vec<char>, l: usize, r: usize) -> usize {
        r
    }
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        if letters[letters.len() - 1] < target {
            return letters[0];
        }
        let mid = (0 + letters.len()) / 2;
        let v = Self::find_between(&letters, 0, mid);
        if v != mid {
            return letters[v];
        }
        let v = Self::find_between(&letters, mid, letters.len());
        if v != letters.len() {
            return letters[v];
        }
        if letters[letters.len() - 1] > target {
            return letters[letters.len() - 1];
        }
        letters[0]
    }
}

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut str = word1.concat();
        let mut i = 0;
        while i < word2.len() {
            let w = &word2[i];
            str = str.replacen(w, "", 1);
            if str.len() == 0 && i < word2.len() - 1 {
                return false;
            }
            i += 1;
        }
        if str.len() != 0 {
            return false;
        }

        true
    }
}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max_len = 1;
        let s_c: Vec<char> = s.chars().collect();
        let mut last = s_c[0];
        let mut t_len = 1;
        let mut i = 1;
        while i < s_c.len() {
            if last == s_c[i] {
                t_len += 1;
            } else {
                max_len = if t_len > max_len { t_len } else { max_len };
                last = s_c[i];
                t_len = 1;
            }
            i += 1;
        }
        max_len = if t_len > max_len { t_len } else { max_len };

        max_len
    }
}

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

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut freq = std::collections::HashMap::new();
        for c in s.chars().into_iter() {
            if let Some(v) = freq.get_mut(&c) {
                *v += 1;
            } else {
                freq.insert(c, 1);
            }
        }
        let mut cons = 0;
        for (_, v) in freq.iter() {
            if cons != 0 && cons != *v {
                return false;
            }
            if cons == 0 {
                cons = *v;
            }
        }

        true
    }
}

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n % 2 == 1 {
            return vec!['a'; n as usize].iter().collect();
        }
        let mut res = vec!['a'; (n - 1) as usize];
        res.push('b');

        res.iter().collect()
    }
}

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let items: Vec<&str> = s.split('0').collect();
        let mut has = false;
        for i in items.iter() {
            if !i.is_empty() && has {
                return false;
            }
            has = true;
        }

        true
    }
}
