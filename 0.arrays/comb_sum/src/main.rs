struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn make(list: Vec<i32>, t: i32, mut path: Vec<i32>, mut res: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            if t == 0 {
                res.push(path);            
                return res;
            } else {
                for (idx, item) in list.iter().enumerate(){                    
                    if item <= &t {
                        path.push(*item);
                        res = make(list[idx..].to_vec(), t - item, path.clone(), res);
                        path.pop();
                    }

                }
            }
            return res;

        }        
        return make(candidates, target, Vec::new(), Vec::new());
    }
}
pub fn main() {
    let ans = Solution::combination_sum([2, 3, 6, 7].to_vec(), 7);
    println!("{:?}", ans);
    // let mut sol = Solution{};
    // sol.combination_sum(Vec::from([2, 3, 6, 7]), 7);

}