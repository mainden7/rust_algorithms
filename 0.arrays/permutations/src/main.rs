struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn run(
            basis: usize,
            list: Vec<i32>,
            mut sub: &mut Vec<i32>,
            mut res: Vec<Vec<i32>>,
        ) -> Vec<Vec<i32>> {
            if sub.len() == basis {
                res.push(sub.clone());
                return res;
            }
            for i in 0..list.len() {
                sub.push(list[i]);

                let mut sl = list[0..i].to_vec();
                sl.extend(list[i + 1..].to_vec());
                res = run(
                    basis,
                    sl,
                    &mut sub,
                    res,
                );
                sub.pop();
            }

            res
        }
        run(nums.len(), nums, &mut vec![], vec![])
    }
}

fn main() {
    let ans = Solution::permute(vec![1, 2, 3]);
    println!("{:?}", ans);
}
