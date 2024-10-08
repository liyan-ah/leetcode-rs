// impl Solution {
//     pub fn distribute_candies(mut candy_type: Vec<i32>) -> i32 {
//         candy_type.sort();
//         let mut ty = 0;
//         let mut last = None;
//         for v in candy_type.iter() {
//             if ty >= candy_type.len() as i32 / 2 {
//                 break;
//             }
//             if last.is_none() {
//                 last = Some(v);
//                 ty += 1;
//                 continue;
//             }
//             if last.unwrap() == v {
//                 continue;
//             }
//             last = Some(v);
//             ty += 1;
//         }
//
//         ty
//     }
// }

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut ty = std::collections::HashSet::new();
        for v in candy_type.iter() {
            if ty.len() >= candy_type.len() / 2 {
                break;
            }
            ty.insert(v);
        }

        ty.len() as i32
    }
}
 