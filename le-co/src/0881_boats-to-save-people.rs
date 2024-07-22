impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        // sort
        let mut people = people.clone();
        people.sort();
        // become two sum question
        let (mut left, mut right) = (0, people.len() - 1);
        let mut boat: i32 = 0;
        while left < right {
            if people[left] + people[right] > limit {
                boat += 1;
                right -= 1;
                continue;
            }
            left += 1;
            right -= 1;
            boat += 1;
        }
        // pick left people
        if left == right {
            boat += 1;
        }
        return boat;
    }
}
