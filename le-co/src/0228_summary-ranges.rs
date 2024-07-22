impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let (mut start, mut end) = (None, None);
        let mut res = vec![];
        for v in nums.iter() {
            match end {
                Some(e) => {
                    if *v == e + 1 {
                        end = Some(*v);
                    } else {
                        res.push(format!("{}->{}", start.unwrap(), e));
                        start = Some(*v);
                        end = None;
                    }
                }
                None => match start {
                    Some(s) => {
                        if *v == s + 1 {
                            end = Some(*v);
                        } else {
                            start = Some(*v);
                            res.push(format!("{s}"));
                        }
                    }
                    None => {
                        start = Some(*v);
                    }
                },
            }
        }
        match start {
            Some(s) => match end {
                Some(e) => {
                    res.push(format!("{s}->{e}"));
                }
                None => {
                    res.push(format!("{s}"));
                }
            },
            None => {}
        }

        res
    }
}
