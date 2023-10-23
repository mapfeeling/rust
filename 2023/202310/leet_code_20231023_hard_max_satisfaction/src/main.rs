struct Solution {}

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable_by(|a, b| b.cmp(a));
        let mut result = 0;
        let mut tmp = 0;
        for &x in &satisfaction {
            tmp += x;
            if tmp <= 0 {
                break;
            }
            result += tmp
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
    let s = Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]);
    println!("{:?}",s);
}
