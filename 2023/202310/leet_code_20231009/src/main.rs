
#[derive(Debug)]
struct Solution {
    number:i32,
}

impl Solution {
    pub fn split_num(&self) -> i32 {
        let mut nums: Vec<i32> = self.number.to_string().chars().map(|c| c as i32 - 48).collect();
        nums.sort();

        let (num1, num2) = nums.iter().fold((0, 0), |(mut n1, n2), input| {
            n1 = n1 * 10 + input;
            (n2, n1)
        });

        num1 + num2
    }
}


fn main() {
    println!("Hello, world!");
    let s = Solution{
        number:4325,
    };
  println!("{}",s.split_num());}


/*
给你一个正整数 num ，请你将它分割成两个非负整数 num1 和 num2 ，满足：
num1 和 num2 直接连起来，得到 num 各数位的一个排列
换句话说，num1 和 num2 中所有数字出现的次数之和等于 num 中所有数字出现的次数
num1 和 num2 可以包含前导 0
请你返回 num1 和 num2 可以得到的和的 最小 值
注意：
num 保证没有前导 0
num1 和 num2 中数位顺序可以与 num 中数位顺序不同
*/