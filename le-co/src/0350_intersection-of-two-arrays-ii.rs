
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();
        let mut intersec: Vec<i32> = vec![];
        let (mut i1, mut i2) = (0 as usize, 0 as usize);
        while (i1 < nums1.len()) && (i2 < nums2.len()) {
            if nums1[i1] < nums2[i2] {
                i1 += 1;
                continue;
            }
            if nums1[i1] > nums2[i2] {
                i2 += 1;
                continue;
            }
            intersec.push(nums1[i1]);
            i1 += 1;
            i2 += 1;
        }
        intersec
    }
}
