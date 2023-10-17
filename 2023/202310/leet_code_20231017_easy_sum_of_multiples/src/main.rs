struct Solution {}

impl Solution {
    pub fn sum_of_multiples_minor(n: i32) -> i32 {
        let mut sum = 0;
        for i in 0..n {
            if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
                sum += i;
            }
        }
        sum
    }
    pub fn sum_of_multiples_std(n: i32) -> i32 {
        (1..=n)
            .filter(|&x| x % 3 == 0 || x % 5 == 0 || x % 7 == 0)
            .sum()
    }
}

fn main() {
    println!("Hello, world!");
}

/*
倍数求和
给你一个正整数 n ，请你计算在 [1，n] 范围内能被 3、5、7 整除的所有整数之和。
返回一个整数，用于表示给定范围内所有满足约束条件的数字之和
*/