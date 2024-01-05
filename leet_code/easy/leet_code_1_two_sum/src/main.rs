use std::collections::HashMap;

struct Solution {}
/*
给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标
你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现
你可以按任意顺序返回答案
*/
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (j, &value) in nums.iter().enumerate() {
            if let Some(&i) = map.get(&(target - value)) {
                return vec![i as i32, j as i32];
            }
            map.insert(value, j);
        }
        unreachable!() // 题目保证有解，循环中一定会 return
    }
}

fn main() {
    println!("Hello, world!");
    let nums:Vec<i32> = Vec::new();
    let nums = vec![2,7,11,15];
    let s = Solution::two_sum(nums, 9);
    println!("{:?}", s)
}
