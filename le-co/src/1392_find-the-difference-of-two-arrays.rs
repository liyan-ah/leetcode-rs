impl Solution {
    pub fn find_difference(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<Vec<i32>> {
        nums1.sort();
        nums2.sort();
        let mut answer_0 = vec![];
        let mut answer_1 = vec![];
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if i >= 1 && nums1[i] == nums1[i - 1] {
                i += 1;
                continue;
            }
            if j >= 1 && nums2[j] == nums2[j - 1] {
                j += 1;
                continue;
            }
            if nums1[i] == nums2[j] {
                i += 1;
                j += 1;
                continue;
            }
            if nums1[i] < nums2[j] {
                answer_0.push(nums1[i]);
                i += 1;
                continue;
            }
            if nums1[i] > nums2[j] {
                answer_1.push(nums2[j]);
                j += 1;
                continue;
            }
        }
        while i < nums1.len() {
            if i >= 1 && nums1[i] == nums1[i - 1] {
                i += 1;
                continue;
            }
            answer_0.push(nums1[i]);
            i += 1;
        }
        while j < nums2.len() {
            if j >= 1 && nums2[j] == nums2[j - 1] {
                j += 1;
                continue;
            }
            answer_1.push(nums2[j]);
            j += 1;
        }

        vec![answer_0, answer_1]
    }
}
