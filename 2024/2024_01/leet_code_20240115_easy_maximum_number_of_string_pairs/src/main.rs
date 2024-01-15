use std::collections::HashSet;

struct Solution{}

/*
给你一个下标从 0 开始的数组 words ，数组中包含 互不相同 的字符串
如果字符串 words[i] 与字符串 words[j] 满足以下条件，我们称它们可以匹配:
字符串 words[i] 等于 words[j] 的反转字符串
0 <= i < j < words.length
请你返回数组 words 中的 最大 匹配数目
注意，每个字符串最多匹配一次
*/
impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut seen = HashSet::new();
        let mut ans = 0;
        for word in words.into_iter() {
            let s = word.chars().rev().collect::<String>();
            println!("{:?}",s);
            if seen.contains(&s) {
                ans+=1;
            }else{
                seen.insert(word);
            }
        }
        ans
    }
}

fn main() {
    let words  = vec![String::from("cd"),String::from("ac"),String::from("dc"),String::from("ca"),String::from("zz")];
    let s = Solution::maximum_number_of_string_pairs(words);
    println!("{:?}",s);
    println!("Hello, world!");
}
