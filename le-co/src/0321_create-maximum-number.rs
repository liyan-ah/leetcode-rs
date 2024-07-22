use std::cmp::Ordering;
impl Solution {
    #[allow(dead_code)]
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max_seq = vec![];
        for len1 in 0..k + 1 {
            if (len1 as usize) > nums1.len() {
                continue;
            }
            if (k - len1) as usize > nums2.len() {
                continue;
            }
            let seq1 = Solution::find_max_subsequence(&nums1, len1);
            let seq2 = Solution::find_max_subsequence(&nums2, k - len1);
            let merge = Solution::merge_seq(&seq1, &seq2);
            if max_seq.len() == 0 {
                max_seq = merge;
                continue;
            }
            let cmp = Solution::compare_seq(&merge, 0, &max_seq, 0);
            match cmp {
                Ordering::Greater => {
                    max_seq = merge;
                }
                _ => {}
            }
        }
        max_seq
    }

    pub fn find_max_subsequence(nums: &Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        if k == 0 {
            return res;
        }
        res.resize(k as usize, 0);
        let mut top = -1;
        let mut remain = nums.len() - (k as usize);
        for pos in 0..nums.len() {
            while (top >= 0) && remain > 0 && res[top as usize] < nums[pos] {
                top -= 1;
                remain -= 1;
            }
            if top < k - 1 {
                top += 1;
                res[top as usize] = nums[pos];
            } else {
                remain -= 1;
            }
        }
        res
    }

    pub fn compare_seq(seq1: &Vec<i32>, p1: usize, seq2: &Vec<i32>, p2: usize) -> Ordering {
        let (mut p1, mut p2) = (p1, p2);
        while p1 < seq1.len() && p2 < seq2.len() {
            if seq1[p1] < seq2[p2] {
                return Ordering::Less;
            }
            if seq1[p1] > seq2[p2] {
                return Ordering::Greater;
            }
            p1 += 1;
            p2 += 1;
        }
        if p1 == seq1.len() && p2 == seq2.len() {
            return Ordering::Equal;
        }
        if p1 < seq1.len() {
            return Ordering::Greater;
        }
        Ordering::Less
    }

    pub fn merge_seq(seq1: &Vec<i32>, seq2: &Vec<i32>) -> Vec<i32> {
        let (mut p1, mut p2) = (0 as usize, 0 as usize);
        let mut res = vec![];
        'merge: loop {
            if p1 >= seq1.len() && p2 >= seq2.len() {
                break 'merge;
            }
            let cmp = Solution::compare_seq(&seq1, p1, &seq2, p2);
            match cmp {
                Ordering::Less => {
                    res.push(seq2[p2]);
                    p2 += 1;
                }
                _ => {
                    res.push(seq1[p1]);
                    p1 += 1;
                }
            }
        }
        res
    }
}
