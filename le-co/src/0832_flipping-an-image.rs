impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn invert(v: i32) -> i32 {
            match v {
                0 => return 1,
                1 => return 0,
                _ => return 0,
            }
        }
        let n = image.len();
        println!("{}", n);
        let mut im: Vec<Vec<i32>> = vec![];
        for i in 0..n {
            let mut v: Vec<i32> = vec![];
            let mut j = n - 1;
            loop {
                v.push(invert(image[i][j]));
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            im.push(v);
        }
        im
    }
}
