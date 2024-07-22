impl Solution {
    #[allow(dead_code)]
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_by(|a, b| (&b[0]).cmp(&a[0]));
        let mut result: Vec<Vec<i32>> = vec![];
        let mut last = 0 as usize;
        let mut pos = 0 as usize;
        while pos < people.len() {
            if people[pos][0] == people[last][0] {
                pos += 1;
                continue;
            }
            // reorder
            people[last..pos].sort_by(|a, b| (&a[1]).cmp(&b[1]));
            while last < pos {
                let idx = people[last][1] as usize;
                result.insert(idx, vec![people[last][0], people[last][1]]);
                last += 1;
            }
            pos += 1;
        }
        people[last..pos].sort_by(|a, b| (&a[1]).cmp(&b[1]));
        while last < pos {
            let idx = people[last][1] as usize;
            result.insert(idx, vec![people[last][0], people[last][1]]);
            last += 1;
        }
        result
    }
}
