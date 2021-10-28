struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut row = vec![false; s.len() + 1];
        row[0] = true;
        for idx in 1..(s.len() + 1) {
            for word in word_dict.iter() {
                if word.len() > idx {
                    continue;
                }
                let sbs = &s[(idx - word.len())..idx];
                if row[idx - word.len()] && sbs == word {
                    row[idx] = true;
                }
            }
        }
        row[s.len()]
    }
}

fn main() {
    let ans = Solution::word_break(
        "leetcode".to_string(),
        vec!["leet", "code"].iter().map(|x| x.to_string()).collect(),
    );
    println!("{:?}", ans);
}
