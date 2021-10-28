struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.len() < 1 {
            return 0;
        }
        let mut row: Vec<u32> = vec![0;s.len() + 1];
        row[0] = 1;       
        
        if s.chars().nth(0).unwrap() == '0' { row[1] = 0} else {row[1] = 1};
        for idx in 2..(s.len() + 1) {
            let char = &s[idx - 1..idx];
            let int_char: u32 = char.parse().unwrap();
            if  int_char > 0 && int_char <= 9 {
                row[idx] += row[idx - 1]
            }  
            let char = &s[idx - 2..idx];
            let int_char: u32 = char.parse().unwrap();
            if int_char > 9 && int_char <= 26 {
                row[idx] += row[idx - 2]
            }
            
        }
        row[s.len()] as i32

    }
}
fn main() {


    let ans = Solution::num_decodings("11106".to_string());
    println!("{}", ans);
    
}
