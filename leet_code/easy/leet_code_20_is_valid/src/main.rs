use std::fs::read;

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec!['0'];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => { stack.push(c) }
                ')' => { if stack.pop().unwrap() != '(' { return false; } }
                ']' => { if stack.pop().unwrap() != '[' { return false; } }
                '}' => { if stack.pop().unwrap() != '{' { return false; } }
                _ => {}
            }
        }
        stack.len() == 1
    }
}

fn main() {
    let s = Solution::is_valid("()[]{}".to_string());
    println!("{:?}", s);
    println!("Hello, world!");
}
