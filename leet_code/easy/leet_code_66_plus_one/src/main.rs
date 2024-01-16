struct Solution {}

// 加一
// 给定一个由 整数 组成的 非空 数组所表示的非负整数，在该数的基础上加一
// 最高位数字存放在数组的首位,数组中每个元素只存储单个数字
// 你可以假设除了整数 0 之外,这个整数不会以零开头
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::from(digits);
        for i in (0..result.len()).rev() {
            println!("{},{}",i, result[i]);
            if result[i] != 9 {
                result[i] += 1;
                return result
            } else {
                result[i] = 0
            }
        }
        result.insert(0, 1);
        result
    }
}



fn main() {
    let s = Solution::plus_one(vec![1, 2, 3]);
    println!("{:?}",s);
    println!("Hello, world!");
}
