struct Solution {}

// 最大单词长度乘积
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mask: Vec<i32> = words
            .iter()
            .map(|word| {
                word.chars()
                    .fold(0, |acc, c| acc | 1 << (c as u8 - 'a' as u8))
            })
            .collect();
        let mut ans = 0;
        for i in 0..mask.len() {
            for j in i + 1..mask.len() {
                if mask[i] & mask[j] == 0 {
                    ans = ans.max(words[i].len() * words[j].len());
                }
            }
        }
        ans as i32
    }
    pub fn max_productAnother(words: Vec<String>) -> i32 {
        let words = words.into_iter().map(|word| (
            word.bytes().fold(0, |a, u| a | (1<<(u-b'a'))),
            word.len() as i32
        )).collect::<Vec<_>>();
        words.iter().fold(0, |ans, ele| {
            ans.max(
                words.iter().filter(|(k, _)| k & ele.0 == 0)
                    .max_by_key(|(_, v)| v)
                    .unwrap_or(&(0, 0)).1
                    * ele.1
            )
        })
    }
}

fn main() {
    println!("Hello, world!");
    let mut  words =Vec::new();
    words.push(String::from("abcw"));
    words.push(String::from("baz"));
    words.push(String::from("foo"));
    words.push(String::from("bar"));
    words.push(String::from("xtfn"));
    words.push(String::from("abcdef"));
    //let words: Vec<String> = vec![];//"abcw","baz","foo","bar","xtfn","abcdef"

    let s = Solution::max_product(words);
    println!("{:?}", s);

}

// 给你一个字符串数组 words ,找出并返回 length(words[i]) * length(words[j]) 的最大值,并且这两个单词不含有公共字母
// 如果不存在这样的两个单词, 返回 0

// 与运算： 0 & 0 =0  0 & 1 =0  1 & 0 =0 1 & 1 =1
// 或运算： 1｜1 =1  1｜0 =1  0｜0 =0  0｜1=1
// 异或运算：0 ⊕ 0 = 0 1 ⊕ 0 = 1 0 ⊕ 1 = 1 1 ⊕ 1 = 0（同为0,异为1）