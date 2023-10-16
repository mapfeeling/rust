use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn single_number_mid(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in nums {
            if let Some(x) = map.get_mut(&n) {
                *x = *x + 1;
            } else {
                map.insert(n, 1);
            }
        }
        for (key, val) in map.iter() {
            if *val == 1 {
                return *key;
            }
        }
        -1
    }
}

fn main() {
    let nums = vec![0, 1, 0, 1, 0, 1, 99];
    let s = Solution::single_number_mid(nums);
    println!("Hello, world!");
    println!("{:?}", s);
}

/*
给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次
请你找出并返回那个只出现了一次的元素
你必须设计并实现线性时间复杂度的算法且使用常数级空间来解决此问题
*/