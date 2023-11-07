use ::std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        words[left as usize..=right as usize]
            .iter()
            .filter(|w| {
                "aeiou".chars().any(|c| w.starts_with(c))
                    && "aeiou".chars().any(|c| w.ends_with(c))
            })
            .count() as i32
    }
}

// 统计范围内的元音字符串数
fn main() {
    println!("Hello, world!");
    let mut words = Vec::new();
    words.push(String::from("hey"));
    words.push(String::from("aeo"));
    words.push(String::from("mu"));
    words.push(String::from("ooo"));
    words.push(String::from("artro"));
    let s = Solution::vowel_strings(words, 1, 4);
    println!("{:?}", s);
}

/*
给你一个下标从 0 开始的字符串数组 words 和两个整数：left 和 right
如果字符串以元音字母开头并以元音字母结尾，那么该字符串就是一个 元音字符串 ,其中元音字母是 'a'、'e'、'i'、'o'、'u'
返回 words[i] 是元音字符串的数目,其中 i 在闭区间 [left, right] 内
*/