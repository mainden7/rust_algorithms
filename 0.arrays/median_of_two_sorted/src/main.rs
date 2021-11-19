use std::arch::x86_64::_SIDD_NEGATIVE_POLARITY;

struct Solution {}

impl Solution {
    pub fn median(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let first;
        let second;
        if nums1.len() <= nums2.len() {
            first = nums1;
            second = nums2;
        } else {
            first = nums2;
            second = nums1;
        }

        let mut low = 0;
        let mut high = first.len();

        let mut part_x;
        let mut part_y;

        let (m, n) = (first.len(), second.len());

        while low <= high {
            part_x = (low + high) / 2;
            part_y = (first.len() + second.len() + 1) / 2 - part_x;

            let (one_left, one_right, two_left, two_right) = match (part_x, part_y) {
                (0, 0) => (
                    -f64::INFINITY,
                    first[part_x] as f64,
                    -f64::INFINITY,
                    second[part_y] as f64,
                ),
                (m, n) => (
                    first[part_x - 1] as f64,
                    f64::INFINITY,
                    second[part_y - 1] as f64,
                    f64::INFINITY,
                ),
            };

            // let one_left = match part_x {
            //     0 => -f64::INFINITY,
            //     _ => first[part_x - 1] as f64,
            // };

            // let one_right = match part_x == first.len() {
            //     true => f64::INFINITY,
            //     false => first[part_x] as f64,
            // };

            // let two_left = match part_y {
            //     0 => -f64::INFINITY,
            //     _ => second[part_y - 1] as f64,
            // };
            // let two_right = match part_y == second.len() {
            //     true => f64::INFINITY,
            //     false => second[part_y] as f64,
            // };

            if one_left <= two_right && two_left <= one_right {
                if (first.len() + second.len()) % 2 == 0 {
                    return (one_left.max(two_left) + one_right.min(two_right)) as f64 / 2.0;
                } else {
                    return one_left.max(two_left) as f64;
                }
            } else if one_left > two_right {
                high = part_x - 1;
            } else {
                low = part_x + 1;
            }
        }
        0.0
    }
}

pub fn main() {
    let a = vec![1, 2];
    let b = vec![3, 4];

    let mut c: Vec<i32> = a.iter().chain(b.iter()).cloned().collect();
    c.sort();
    let ans = Solution::median(a, b);
    println!("{:?}", ans);
    println!("{:?}", (c.len() / 2));
}
