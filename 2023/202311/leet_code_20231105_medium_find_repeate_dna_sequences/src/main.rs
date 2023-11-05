use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }
        let mut map: HashMap<&str, bool> = HashMap::new();
        for i in 0..s.len() - 9 {
            let slice = &s[i..i + 10];
            if map.contains_key(slice) {
                map.insert(slice, true);
            } else {
                map.insert(slice, false);
            }
        }
        map.keys().filter(|&s| map[s]).map(|s| s.to_string()).collect()
    }
}

fn main() {
    println!("Hello, world!");
    let mut str = String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT");
    let s = Solution::find_repeated_dna_sequences(str);
    println!("{:?}", s);
}

/*
DNA序列 由一系列核苷酸组成,缩写为 'A', 'C', 'G' 和 'T'
例如，"ACGAATTCCG" 是一个 DNA序列
在研究 DNA 时，识别 DNA 中的重复序列非常有用
给定一个表示 DNA序列 的字符串 s ,返回所有在 DNA 分子中出现不止一次的 长度为 10 的序列(子字符串)
你可以按 任意顺序 返回答案
*/