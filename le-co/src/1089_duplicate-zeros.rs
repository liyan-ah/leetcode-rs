impl Solution {
    #[allow(dead_code)]
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut num = 0 as usize;
        // find 0 num
        let mut pos = 0 as usize;
        let mut length = arr.len() - 1;
        while pos < arr.len() - num {
            if arr[pos] == 0 {
                if pos + num == length {
                    arr[length] = 0;
                    length -= 1;
                    break;
                }
                num += 1;
            }
            pos += 1;
        }
        // start to move
        pos = length - num;
        let mut end = length;
        while num > 0 && pos >= 0 {
            if arr[pos] == 0 {
                arr[end] = 0;
                arr[end - 1] = 0;
                end -= 2;
                pos -= 1;
                num -= 1;
                continue;
            }
            arr[end] = arr[pos];
            pos -= 1;
            end -= 1;
        }
        return;
    }
}
