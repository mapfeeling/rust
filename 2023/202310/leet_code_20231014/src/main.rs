struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            res ^= *nums[i];
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
