use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut storage: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut transformed_word: Vec<char> = s.chars().collect();
            transformed_word.sort();
            let sorted_word = transformed_word.iter().collect::<String>();
            match storage.entry(sorted_word) {
                Entry::Vacant(e) => {
                    e.insert(vec![s]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(s);
                }
            }
        }
        storage.values().cloned().collect()
    }
}

fn main() {
    let ans = Solution::group_anagrams(
        vec!["eat", "tea", "tan", "ate", "nat", "bat"].iter().map(|&x| x.to_string()).collect()
    );
    println!("{:?}", ans);
}