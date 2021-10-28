use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: HashSet<Vec<i32>> = HashSet::new();

        let pos: Vec<i32> = nums.iter().filter(|&x| x > &0).cloned().collect();
        let neg: Vec<i32> = nums.iter().filter(|&x| x < &0).cloned().collect();
        let zeroes: Vec<i32> = nums.iter().filter(|&x| x == &0).cloned().collect();

        let pos_set: HashSet<i32> = HashSet::from_iter(pos.clone().into_iter());
        let neg_set: HashSet<i32> = HashSet::from_iter(neg.clone().into_iter());

        if zeroes.len() > 2 {
            ans.insert(vec![0, 0, 0]);
        }

        if !zeroes.is_empty() {
            for p in pos_set.iter() {
                if neg_set.contains(&-p) {
                    let mut v = vec![*p, -p, 0];
                    v.sort();
                    ans.insert(v);
                }
            }
        }


        for i in 0..neg.len() {
            for j in i + 1..neg.len() {
                let target = -(neg[i] + neg[j]);
                if pos_set.contains(&target) {
                    let mut v = vec![neg[i], neg[j], target];
                    v.sort();
                    ans.insert(v);
                }
            }
        }

        for i in 0..pos.len() {
            for j in i + 1..pos.len() {
                let target = -(pos[i] + pos[j]);
                if neg_set.contains(&target) {
                    let mut v = vec![pos[i], pos[j], target];
                    v.sort();
                    ans.insert(v);
                }
            }
        }

        ans.into_iter().collect()
    }
}

fn main() {
    let ans = Solution::three_sum(vec![1, 1, -2]);
    println!("{:?}", ans);
}
