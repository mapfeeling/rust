struct  Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            res ^= nums[i];
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
    let nums = vec![2,3,4,10,2,4,3];
    let  s = Solution::single_number(nums);
    println!("{}", s);
}
