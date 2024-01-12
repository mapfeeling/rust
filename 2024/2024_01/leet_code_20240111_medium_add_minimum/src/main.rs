struct Solution {}

use std::collections::VecDeque;

/*
给你一个字符串 word ，你可以向其中任何位置插入 "a"、"b" 或 "c" 任意次，返回使 word 有效 需要插入的最少字母数
如果字符串可以由 "abc" 串联多次得到，则认为该字符串 有效
*/
impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut q = VecDeque::new();
        let mut cnt = 0;
        for c in word.chars() {
            cnt += if c == 'a' {
                match q.back() {
                    Some('a') => 2,
                    Some('b') => 1,
                    Some('c') => 0,
                    _ => 0,
                }
            } else if c == 'b' {
                match q.back() {
                    Some('a') => 0,
                    Some('b') => 2,
                    Some('c') => 1,
                    _ => 1,
                }
            } else {
                match q.back() {
                    Some('a') => 1,
                    Some('b') => 0,
                    Some('c') => 2,
                    _ => 2,
                }
            };
            q.push_back(c);
        }
        cnt + match q.back() {
            Some('a') => 2,
            Some('b') => 1,
            Some('c') => 0,
            _ => 0,
        }
    }
}



fn main() {
    let s = Solution::add_minimum(String::from("aaabbbc"));
    println!("所需要的最小构造数:{:?}", s);
}
