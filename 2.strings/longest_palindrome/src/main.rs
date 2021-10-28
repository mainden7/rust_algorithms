use std::env::join_paths;

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let x: String = s.split("").collect::<Vec<&str>>().join("|");
        let pr = vec![0;x.len()];
        let (mut center, mut radius) = (0, 0);
        while center < x.len() {}

        println!("{:?}", x);
        "".to_string()
    }
}

fn main() {
    let ans = Solution::longest_palindrome("asd".to_string());
    println!("{:?}", ans);
}
