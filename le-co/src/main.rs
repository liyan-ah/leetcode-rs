mod greedy;
mod two_pointers;
//use two_pointers::Solution;
mod dp;
mod hs;
mod ls;
mod mt;
mod se;
mod so_array;
mod str_s;
mod tr;

struct Solution {}

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut change = 0;
        let mut last = None;
        let mut cur;
        for iter in s.as_bytes().iter() {
            if last.is_none() {
                last = Some(iter.clone());
                continue;
            }
            cur = iter.clone();
            if last
                .unwrap()
                .to_ascii_lowercase()
                .eq(&cur.to_ascii_lowercase())
            {
                println!("{} eq {}", last.unwrap(), cur);
                continue;
            }
            last = Some(cur);
            change += 1;
        }
        change
    }
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 0;
    println!("{}", se::Solution::search_insert(nums, target));
    //let s = String::from("aa");
    //println!("{:?}", dp::Solution::partition(s));
    // println!("{:?}", Solution::count_key_changes(String::from("aAbBcC")));
    //let piles = vec![3, 7, 2, 3];
    //println!("{:?}", dp::Solution::stone_game(piles));
    //let n = 25;
    //println!("{:?}", dp::Solution::tribonacci(n));
    //println!("{:?}", dp::Solution::get_maximum_generated(n));
    //println!("{:?}", dp::Solution::fib(n));
    //let n = 5;
    //println!("{:?}", dp::Solution::count_bits(n));
    //let s = String::from("pineapplepenapple");
    //let word_dict: Vec<String> = vec![
    //    String::from("apple"),
    //    String::from("pen"),
    //    String::from("applepen"),
    //    String::from("pine"),
    //    String::from("pineapple"),
    //];
    //println!("{:?}", dp::Solution::word_break_2(s, word_dict));
    //let nums = vec![1];
    //println!("{:?}", dp::Solution::find_target_sum_ways(nums, 1));
    //let coins = vec![1, 2, 5];
    //println!("{:?}", dp::Solution::change(5, coins));
    //let nums = vec![2, 3, 4, 1];
    //println!("{:?}", dp::Solution::rob_2(nums));
    //let prices = vec![3, 2, 6, 5, 0, 3];
    //println!("{:?}", dp::Solution::max_profit_k(2, prices));
    //let prices = vec![1, 3, 7, 5, 10, 3];
    //println!("{:?}", dp::Solution::max_profit_fee(prices, 3));
    //let prices = vec![1];
    //println!("{:?}", dp::Solution::max_profit_cold(prices));
    //let nums1 = vec![0, 1, 1, 1, 1];
    //let nums2 = vec![1, 0, 1, 0, 1];
    //println!("{:?}", dp::Solution::find_length(nums1, nums2));
    //let prices = vec![7, 6, 5, 4, 3, 1];
    //println!("{:?}", dp::Solution::max_profit(prices));
    //let arr = vec![5, 0, -1, -1, 4];
    //println!("{:?}", dp::Solution::maximum_sum(arr));
    //let nums = vec![-1, -2, -3, 0, 4];
    //println!("{:?}", dp::Solution::get_max_len(nums));
    //let nums = vec![-2, 3, -4];
    //println!("{:?}", dp::Solution::max_product(nums));
    //let nums = vec![-3, 2, -4];
    //println!("{:?}", dp::Solution::max_absolute_sum(nums));
    //let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    //println!("{:?}", dp::Solution::min_cost_climbing_stairs(cost));
    //let (s1, s2) = (String::from("delete"), String::from("leet"));
    //println!("{:?}", dp::Solution::minimum_delete_sum(s1, s2));
    //let (word1, word2) = (String::from("leetcode"), String::from("etco"));
    //println!("{:?}", dp::Solution::min_distance(word1, word2));
    //let s = String::from("cbbd");
    //println!("{:?}", dp::Solution::longest_palindrome_subseq(s));
    //let (s1, s2, s3) = (
    //    String::from("aabbc"),
    //    String::from("dd"),
    //    String::from("aadbbdc"),
    //);
    //println!("{:?}", dp::Solution::is_interleave(s1, s2, s3));
    //let s = String::from("adceb");
    //let p = String::from("adce.*");
    //println!("{}", greedy::Solution::is_match_r(s, p));
    //let nums = vec![-4, -2, -3];
    //println!(
    //    "{}",
    //    greedy::Solution::largest_sum_after_k_negations(nums, 4)
    //);
    //let position = vec![1, 10000000];
    //println!("{}", greedy::Solution::min_cost_to_move_chips(position));
    //let nums = vec![7, 2, 5, 10, 8];
    //println!("{}", greedy::Solution::split_array(nums, 2));
    //let nums1 = vec![3, 4, 6, 5];
    //let nums2 = vec![9, 1, 2, 5, 8, 3];
    //let max_seq = Solution::max_number(nums1, nums2, 5);
    //println!("{:?}", max_seq);
    //let s = String::from("cbacdcbc");
    //println!("{}", greedy::Solution::remove_duplicate_letters(s));
    //let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    //println!("{}", greedy::Solution::find_min_arrow_shots(points));
    //let nums = vec![3, 30, 34, 5, 9];
    //println!("{}", greedy::Solution::largest_number(nums));
    //let nums = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
    //println!("{}", greedy::Solution::wiggle_max_length(nums));
    //let nums = vec![1, 2, 1, 2, 3];
    //println!("{}", greedy::Solution::largest_perimeter(nums));
    //let bills = vec![5, 5, 5, 10, 10, 20];
    //let ok = greedy::Solution::lemonade_change(bills);
    //println!("{}", ok);
    //let ratings = vec![1, 0, 2];
    //println!("{}", greedy::Solution::candy(ratings));
    //let nums = vec![6, 2, 6, 5, 1, 2];
    //println!("max sum: {}", greedy::Solution::array_pair_sum(nums));
    //let (k, w) = (3, 0);
    //let (profits, capital) = (vec![1, 2, 3], vec![0, 1, 1]);
    //println!(
    //    "max capital is: {}",
    //    greedy::Solution::find_maximized_capital(k, w, profits, capital)
    //);
    //let g: Vec<i32> = vec![2, 3, 4];
    //let s: Vec<i32> = vec![1, 2, 3];
    //let content = greedy::Solution::find_content_children(g, s);
    //println!("content: {}", content);
    //let nums: Vec<i32> = vec![2, 1, 5, 0, 4, 6];
    //println!("{}", greedy::Solution::increasing_triplet(nums));
    //let s = String::from("bcabc");
    //let after = greedy::Solution::remove_duplicate_letters(s);
    //println!("{}", after);
    //let s = String::from("cabb");
    //println!("length: {}", greedy::Solution::longest_palindrome(s));
    //let gas: Vec<i32> = vec![2, 3, 4];
    //let cost: Vec<i32> = vec![3, 4, 3];
    //let res = greedy::Solution::can_complete_circuit(gas, cost);
    //println!("{}", res);
    //let prices: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    //let profit = greedy::Solution::max_profit(prices);
    //println!("{}", profit);
    //let nums: Vec<i32> = vec![9, 12, 5, 10, 14, 3, 10];
    //let new = two_pointers::Solution::pivot_array(nums, 10);
    //println!("{:?}", new);
    //let first_list: Vec<Vec<i32>> = vec![vec![1, 3], vec![5, 9]];
    //let second_list: Vec<Vec<i32>> = vec![];
    //let new_list = two_pointers::Solution::interval_intersection(first_list, second_list);
    //println!("{:?}", new_list);
    //let nums: Vec<i32> = vec![-5, -3, -2, -1];
    //let nums = two_pointers::Solution::sorted_squares(nums);
    //println!("{:?}", nums);
    //let s = String::from("DDI");
    //let perm = two_pointers::Solution::di_string_match(s);
    //println!("{:?}", perm);
    //let nums: Vec<i32> = vec![3, 2];
    //let new_nums = two_pointers::Solution::sort_array_by_parity_ii(nums);
    //println!("{:?}", new_nums);
    //let s = String::from("Test1ng-Leet=code-Q!");
    //let rev = two_pointers::Solution::reverse_only_letters(s);
    //println!("{}", rev);
    //let (s, t) = (String::from("ab#c"), String::from("ad#c"));
    //let is_match = two_pointers::Solution::backspace_compare(s, t);
    //println!("match:{}", is_match);
    //let s = String::from("loveleetcode");
    //let c = 'e';
    //let dist = two_pointers::Solution::shortest_to_char(s, c);
    //println!("{:?}", dist);
    //let (start, end) = (String::from("RXXLRXRXL"), String::from("XRLXXRRLX"));
    //let can = two_pointers::Solution::can_transform(start, end);
    //println!("{}", can);
    //let s = String::from("eccbbbbdec");
    //let label_vec = two_pointers::Solution::partition_labels(s);
    //println!("s length is: {:?}", label_vec);
    //let nums:Vec<i32> = vec![1,3,1];
    //println!("{}", two_pointers::Solution::smallest_distance_pair(nums, 2));
    //let c = 17 as i32;
    //println!("{}", two_pointers::Solution::judge_square_sum(c));
    //let nums:Vec<i32> = vec![2,2,3,4];
    //let num = two_pointers::Solution::triangle_number(nums);
    //println!("{}",num);
    //let houses:Vec<i32> = vec![1,2,3,4];
    //let heaters:Vec<i32> = vec![1,2];
    //let radiu = two_pointers::Solution::find_radius(houses, heaters);
    //println!("{}", radiu);
    //let nums:Vec<i32> = vec![3,1,2];
    //println!("{}", two_pointers::Solution::circular_array_loop(nums));
    //let s = String::from("10101");
    //println!("{}", two_pointers::Solution::count_binary_substrings(s));
    //let numbers:Vec<i32> = vec![2,7,11,15];
    //println!("target pos is: {:?}", two_pointers::Solution::two_sum(numbers, 9));
    //let head = two_pointers::Solution::
    //let mut nums:Vec<i32> = vec![1,1,1,1,2,2,2,3,3,3];
    //let k = two_pointers::Solution::remove_duplicates(&mut nums);
    //println!("nums: {:?}, k: {}", nums, k);
    //let mut head = Some(Box::new(two_pointers::ListNode::new(1)));
    //head = two_pointers::Solution::rotate_right(head, 2);
    //println!("{:?}", head);
    //let (version1, version2) = (String::from("1.01.0"), String::from("1.001"));
    //let comp = two_pointers::Solution::compare_version(version1, version2);
    //println!("comp result: {}", comp);
    //let nums1:Vec<i32> = vec![4,9,5];
    //let nums2:Vec<i32> = vec![9,4,9,8,4];
    //let intersec = two_pointers::Solution::intersect(nums1, nums2);
    //println!("{:?}", intersec);
    //let head = Some(Box::new(two_pointers::ListNode::new(1)));
    //let pali = two_pointers::Solution::is_palindrome(head);
    //println!("pali is: {}", pali);
    //let nums:Vec<i32> = vec![1,3,1,5,4];
    //let k:i32 = 0;
    //let num = two_pointers::Solution::find_pairs(nums, k);
    //println!("pair num is: {}", num);
    //let (s, t) = (String::from("axc"), String::from("ahbgdc"));
    //let is_sub = two_pointers::Solution::is_subsequence(s, t);
    //println!("is sub is: {}", is_sub);
    //let pos = two_pointers::Solution::str_str(String::from("aababaa"), String::from("ab"));
    //println!("pos is: {}", pos);
    //let nums: Vec<i32> = vec![-1,-1,-1,-1];
    //let sub = two_pointers::Solution::find_unsorted_subarray(nums);
    //println!("subarray is {}", sub);
    //let nums:Vec<i32> = vec![3,1,3,4,2];
    //let dup = two_pointers::Solution::find_duplicate(nums);
    //println!("dumplicate number is: {}", dup);
    //let height:Vec<i32> = vec![4,2,0,3,2,5];
    //let water = two_pointers::Solution::trap(height);
    //println!("water is: {}", water);
    //let s = String::from("((())())");
    //println!("s is: {}", s);
    //let score = so_array::Solution::score_of_parentheses(s);
    //println!("score: {}", score);
    //let mut nums: Vec<i32> = vec![2, 1, 0];
    //Solution::sort_colors(&mut nums);
    //println!("after sort, colors are {:?}", nums);
    //let s = String::from("");
    //println!("string: {}", s);
    //let pali = Solution::valid_palindrome(s);
    //println!("pali: {}", pali);
    //let people: Vec<i32> = vec![3,5,3,4];
    //let limit:i32 = 5;
    //println!("people: {:?}", people);
    //let boat = Solution::num_rescue_boats(people, limit);
    //println!("needs boat: {}", boat);
    //let head = Some(Box::new(two_pointers::ListNode {
    //    val: 1,
    //    next: Some(Box::new(ListNode {
    //        val: 2,
    //        next: Some(Box::new(ListNode {
    //            val: 3,
    //            next: Some(Box::new(ListNode {
    //                val: 4,
    //                next: Some(Box::new(ListNode {
    //                    val: 5,
    //                    next: Some(Box::new(ListNode { val: 6, next: None })),
    //                })),
    //            })),
    //        })),
    //    })),
    //}));

    //let head = Some(Box::new(two_pointers::ListNode {
    //    val: 1,
    //    next: Some(Box::new(ListNode {val:2, next:None}))}));
    //println!("head's value is: {:?}", head);
    //let mid_head = Solution::middle_node(head);
    //println!("mid's value is: {:?}", mid_head);
    //let mut chars: Vec<char> = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    //let mut chars: Vec<char> = vec![
    //    'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    //];
    //let comp_len = two_pointers::Solution::compress(&mut chars);
    //println!("comp len is: {}", comp_len);
    //println!("new vec is: {:?}", chars);
    //let s = "hello world".to_string();
    //let re_s = rev_wds_3::Solution::reverse_words(s);
    //println!("after reverse, {}", re_s);
    //let vec1: Vec<i32> = vec![4, 9, 5];
    //let vec2: Vec<i32> = vec![9, 4, 9, 8, 4];
    //let sec = intersection_arrary::Solution::intersection(vec1, vec2);
    //println!("intersec info is: {:?}", sec);
    //let str: String = String::from("leetcode");
    //let str = reverse_volwel::Solution::reverse_volwel(str);
    //println!("after reverse: {:?}", str);
    //let mut s:Vec<char> = vec!['H'];
    //reverse_string::Solution::reverse_string(&mut s);
    //println!("after reverse, {:?}", s);
    //let str:String = " ".to_string();
    //let str:String = "A man, a plan, a canal: Panama".to_string();
    //let str:String = "race a car".to_string();
    //let str: String = "".to_string();
    //let res = palindrome::Solution::is_palindrome(str);
    //println!("palindrome res is: {}", res);

    // happy_number
    //let h_res = happy_number::Solution::is_happy(19);
    //println!("happy_number is happy: {}", h_);
    //let matrix = vec![
    //    vec!['1', '0', '1', '0', '0'],
    //    vec!['1', '0', '1', '1', '1'],
    //    vec!['1', '1', '1', '1', '1'],
    //    vec!['1', '0', '0', '1', '0'],
    //];
    //println!("len: {}", matrix.len());
    //println!("square: {}", dp::Solution::maximal_square(matrix));

    //println!("ways to step: {}", dp::Solution::ways_to_step(1));
}
