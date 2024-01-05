struct Solution {}
/*
给你一个整数 x ，如果 x 是一个回文整数，返回 true ;否则,返回 false
回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数
*/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut nums_box = Vec::new();
        let mut value = x;
        let mut count = 0;
        while value >= 10 {
            let target = value % 10;
            nums_box.push(target);
            value = (value - target) / 10;
        }
        nums_box.push(value);
        for i in 0..nums_box.len() {
            if nums_box[i] != nums_box[nums_box.len() - i - 1] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let s = Solution::is_palindrome(1121);
    println!("{:?}", s);
    println!("Hello, world!");
}
