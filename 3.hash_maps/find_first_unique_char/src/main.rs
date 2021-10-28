use std::collections::HashMap;

struct Solution {}


impl Solution {
    pub fn first_unique_char(s: String) -> i32 {
        let mut counter: HashMap<char, i32> = HashMap::new();
        for (_idx, char) in s.chars().enumerate() {
            counter.insert(char, *counter.get(&char).unwrap_or(&0) + 1);
        }
        for (idx, char) in s.chars().enumerate() {
            if counter[&char] == 1 {
                return idx as i32;
            }
        }
        -1
    }
}

fn main() {
    let ans = Solution::first_unique_char("asdasdkaposkdpasdlkpaoskdpaoskd".to_string());
    println!("{:?}", ans);
}