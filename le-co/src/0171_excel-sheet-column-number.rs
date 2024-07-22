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
