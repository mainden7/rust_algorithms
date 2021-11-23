use std::collections::{BTreeMap, HashMap};
struct Solution {}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl Solution {
    /*
    "2": "abc",
    "3": "def",
    "4": "ghi",
    "5": "jkl",
    "6": "mno",
    "7": "pqrs",
    "8": "tuv",
    "9": "wxyz",
    */
    pub fn letter_combinations(digits: String) ->  Vec<String> {
        let mut chars = HashMap::new();
        chars.insert("2", vec!["a", "b", "c"]);
        // chars.insert("3", ["d", "e", "f"]);
        // chars.insert("4", ["g", "h", "i"]);
        // chars.insert("5", ["j", "k", "l"]);
        // chars.insert("6", ["m", "n", "o"]);
        // chars.insert("7", ["p", "q", "r", "s"]);
        // chars.insert("8", ["t", "u", "v"]);
        // chars.insert("9", ["w", "x", "y", "z"]);

        let list = chars.get("2");

        for l in list.iter() {
            println!("{:?}", l);
        }
        return ["1".to_string(), "2".to_string()].to_vec();


    }
    
}

pub fn main() {
    let ans = Solution::letter_combinations("23".to_string());
    println!("{:?}", ans);
}