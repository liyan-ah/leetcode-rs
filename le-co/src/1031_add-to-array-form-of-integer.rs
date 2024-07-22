impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        num.reverse();
        let mut addition = 0;
        let mut i = 0;
        while k > 0 || addition > 0 {
            let v = k % 10;
            k = k / 10;
            let mut new_v = if i >= num.len() { 0 } else { num[i] };
            let sum = new_v + v + addition;
            new_v = sum % 10;
            addition = sum / 10;

            if i >= num.len() {
                num.push(new_v);
            } else {
                num[i] = new_v;
            }

            i += 1;
        }

        num.reverse();
        num
    }
}
