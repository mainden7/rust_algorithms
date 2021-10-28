use std::collections::hash_map::Entry;
use std::collections::HashMap;

// https://leetcode.com/problems/longest-substring-without-repeating-characters/
struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut chars = [-1; 256];
        let mut ans = 0;
        let mut m = 0;
        for i in 0..s.len() {
            let ec = s.chars().nth(i).unwrap() as usize;
            m = m.max(chars[ec] + 1);
            chars[ec] = i as i32;
            if i as i32 >= m {
                ans = ans.max(i as i32 - m + 1);
            }
        }
        ans as i32
        // if s.len() <= 1 {
        //     return s.len() as i32;
        // }
        // let mut start = 0;
        // let mut end = 1;
        // let mut ans: i32 = 0;
        // let mut storage: HashMap<char, usize> = HashMap::new();
        // storage.insert(s.chars().nth(0).unwrap(), 0);
        //
        // while end < s.len() {
        //     let end_char = s.chars().nth(end).unwrap();
        //     match storage.entry(end_char) {
        //         Entry::Occupied(v) => {
        //             if v.get() >= &start {
        //                 start = v.get() + 1;
        //             }
        //             storage.insert(end_char, end);
        //             ans = ans.max(end as i32 - start as i32 + 1);
        //             end += 1;
        //         }
        //         Entry::Vacant(v) => {
        //             v.insert(end);
        //             ans = ans.max(end as i32 - start as i32 + 1);
        //             end += 1;
        //         }
        //     }
        // }
        // ans
    }
}

fn main() {
    let ans = Solution::length_of_longest_substring(" ".to_string());
    println!("{:?}", ans);
}
