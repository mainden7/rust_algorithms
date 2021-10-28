use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut storage: HashMap<i32, i32> = HashMap::new();

        for (idx, ele) in nums.iter().enumerate() {
            match storage.entry(*ele) {
                Entry::Occupied(o) => return vec![*o.get(), idx as i32],
                Entry::Vacant(_) => storage.insert(target - ele, idx as i32),
            };
        }
        vec![-1, -1]
    }
}

fn main() {
    let ans = Solution::two_sum(vec![3, 3], 6);
    println!("{:?}", ans);
}
