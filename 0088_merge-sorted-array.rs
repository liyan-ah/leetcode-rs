//pub struct Solution{
//    
//}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m_:usize = (m-1) as usize;
        let mut n_:usize = (n-1) as usize;
        let mut s: usize = (m+n-1) as usize;
        if nums2.len() == 0{
            return;
        }
        if m == 0{
            while n_ >= 0{
                nums1[s] = nums2[n_];
                if n_ == 0{
                    break;
                }
                n_ -= 1;
                s -= 1;
            }
            return;
        }
        while s >= 0 && m_ >= 0 && n_ >= 0{
            println!("s:{}, m_:{}, n_:{}", s, m_, n_);
            // find max of nums1[m_] and nums2[n_]
            let mut max_: i32 = nums1[m_];
            // nums2 got the max, move the pos and set value
            if max_ < nums2[n_]{
                max_ = nums2[n_];
                nums1[s] = max_;
                s -= 1;
                if n_ == 0{
                    while m_ >= 0{
                        nums1[s] = nums1[m_];
                        if m_ == 0{
                            break;
                        }
                        s -= 1;
                        m_ -=1;
                    }
                    break;
                }
                n_ -= 1;
                continue;
            }
            // nums1 got the max, move the pos and set value
            nums1[s] = max_;
            s-= 1;
            if m_ == 0{
                while n_ >= 0{
                    nums1[s] = nums2[n_];
                    if n_ == 0{
                        break;
                    }
                    s -= 1;
                    n_ -= 1;
                }
                break;
            }
            m_ -= 1;
        }
        return;
    }
}

//fn main(){
//    let mut nums1 = vec![2, 0];
//    let m:i32 = 1;
//    let mut nums2 = vec![1];
//    let n:i32 = 1;
//    Solution::merge(&mut nums1, m, &mut nums2, n);
//    println!("nums1 is {:?}", nums1)
//}
