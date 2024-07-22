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
