struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}

fn main() {
    let s = Solution::search_insert(vec![1,3,5,6],5);
    println!("{:?}",s);
    println!("Hello, world!");
}
