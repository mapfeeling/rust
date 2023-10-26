struct Solution {}

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .filter(|&d| num % d as i32 == 0)
            .count() as i32
    }
}

fn main() {
    println!("Hello, world!");
    let s = Solution::count_digits(1248);
    println!("{:?}", s);
}

/*
给你一个整数 num ，返回 num 中能整除 num 的数位的数目。
如果满足 nums % val == 0 ，则认为整数 val 可以整除 nums
*/