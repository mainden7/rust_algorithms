struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(n) => n as i32,
            None => -1,
        }
        // if &haystack.len() < &needle.len() {
        //     return -1;
        // }
        // for i in 0..(&haystack.len() - &needle.len() + 1) {
        //     if &haystack[i..(i + &needle.len())] == &needle {
        //         return i as i32;
        //     }
        // }
        // -1
    }
}

fn main() {
    let ans = Solution::str_str("hello".to_string(), "ll".to_string());
    println!("{}", ans);
}
