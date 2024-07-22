impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c_sqrt = (c as f64).sqrt();
        let (mut a, mut b) = (0 as i32, c_sqrt as i32);
        while a <= b{
            // 2147483600 case would be out of i32
            let square = (a*a) as i64 + (b*b) as i64;
            if square == (c as i64){
                return true;
            }
            if square < (c as i64){
                a += 1;
                continue;
            }
            b -= 1;
        }
        println!("a:{}, b:{}", a, b);
        println!("square:{}", a*a+b*b);
        false
    }
}
