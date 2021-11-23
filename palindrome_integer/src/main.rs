struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        } else {
            let str_int = x.to_string();
            if str_int == str_int.chars().rev().collect::<String>() {
                return true;
            } else {
                return false;
            }
        }
    }
}

fn main() {
    let ans = Solution::is_palindrome(121);
    println!("{}", ans);
}
