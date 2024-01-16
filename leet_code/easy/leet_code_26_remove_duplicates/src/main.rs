use std::collections::HashSet;

struct Solution {}

// 删除有序数组中的重复项
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let s = Solution::remove_duplicates(&mut nums);
    println!("{:?}", s);
    println!("Hello, world!");
}
