impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        if flowerbed.len() == 1 && flowerbed[0] == 0 && n == 1 {
            return true;
        }
        for i in 0..flowerbed.len() {
            if n == 0 {
                return true;
            }
            if (i >= 1 && flowerbed[i - 1] == 1)
                || flowerbed[i] == 1
                || (i < flowerbed.len() - 1 && flowerbed[i + 1] == 1)
            {
                continue;
            }
            if i == flowerbed.len() - 1 || (i < (flowerbed.len() - 1) && flowerbed[i + 1] == 0) {
                n -= 1;
                flowerbed[i] = 1;
            }
        }
        println!("{:?}", flowerbed);
        return if n == 0 { true } else { false };
    }
}
