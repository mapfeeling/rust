struct Solution {}

// 找出字符串中的第一个匹配项目的下标
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(index) => index as i32,
            None => -1,
        }
    }
}

fn main() {
    let s = Solution::str_str(String::from("sadbutsad"), String::from("ts"));
    println!("{:?}", s);
    println!("Hello, world!");
}
