struct Solution {}

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut result = 0i32;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let dummy = nums[i] * nums[j];
                *map.entry(dummy).or_insert(0) += 1;
            }
        }
        for val in map.values() {
            result += val * (val - 1) / 2;
        }
        result << 3
    }
}

fn main() {
    println!("Hello, world!");
}

/*
同积元组
给你一个由 不同 正整数组成的数组 nums
请你返回满足 a * b = c * d 的元组 (a, b, c, d) 的数量
其中 a、b、c 和 d 都是 nums 中的元素，且 a != b != c != d
*/

/*
示例 1：
输入：nums = [2,3,4,6]
输出：8
解释：存在 8 个满足题意的元组：
(2,6,3,4) , (2,6,4,3) , (6,2,3,4) , (6,2,4,3)
(3,4,2,6) , (4,3,2,6) , (3,4,6,2) , (4,3,6,2)

示例 2：
输入：nums = [1,2,4,5,10]
输出：16
解释：存在 16 个满足题意的元组：
(1,10,2,5) , (1,10,5,2) , (10,1,2,5) , (10,1,5,2)
(2,5,1,10) , (2,5,10,1) , (5,2,1,10) , (5,2,10,1)
(2,10,4,5) , (2,10,5,4) , (10,2,4,5) , (10,2,4,5)
(4,5,2,10) , (4,5,10,2) , (5,4,2,10) , (5,4,10,2)
*/