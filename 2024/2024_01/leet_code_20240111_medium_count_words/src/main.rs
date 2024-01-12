/*
给你两个字符串数组 words1 和 words2 ，请你返回在两个字符串数组中 都恰好出现一次 的字符串的数目
*/
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        words1.into_iter().for_each(|x| *map1.entry(x).or_insert(0) += 1);
        words2.into_iter().for_each(|x| *map2.entry(x).or_insert(0) += 1);
        let mut count = 0;

        for (k, v) in map1 {
            if let Some(&u) = map2.get(&k) {
                if v == 1 && u == 1 {
                    count += 1;
                };
            }
        }
        count
    }
}

fn main() {
    let words1 = vec![String::from("leetcode"), String::from("is"), String::from("amazing"), String::from("as"), String::from("is")];
    let words2 = vec![String::from("amazing"), String::from("leetcode"), String::from("is")];
    let s = Solution::count_words(words1, words2);
    println!("{:?}", s);
    println!("Hello, world!");
}
