struct Solution {}

// 最后一个单词的长度
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if s.trim().len() == 0 {
            return 0;
        }
        s.split_whitespace().last().unwrap().len() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
