struct MedianFinder {
    nums:Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        MedianFinder{
            nums: Vec::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        self.nums.push(num);
    }
    
    fn find_median(&mut self) -> f64 {
        self.nums.sort();
        let num_len = self.nums.len();
        if num_len % 2 == 0{
            let lower = num_len / 2 - 1;
            let upper = lower + 1 ;
            let mid = (self.nums[lower]+self.nums[upper]) as f64 / (2 as f64);
            return mid;
        }
        let mid = num_len/2;
        return self.nums[mid] as f64;

    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
