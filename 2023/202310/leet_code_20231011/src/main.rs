use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let n = student_id.len();
        let ps = positive_feedback.iter().collect::<HashSet<&String>>();
        let ns = negative_feedback.iter().collect::<HashSet<&String>>();
        let mut map = HashMap::new();
        for i in 0..n {
            let id = student_id[i];
            let mut count = 0;
            for s in report[i].split(' ') {
                let s = &s.to_string();
                if ps.contains(s) {
                    count += 3;
                } else if ns.contains(s) {
                    count -= 1;
                }
            }
            map.insert(id, count);
        }
        let mut t = map.into_iter().collect::<Vec<(i32, i32)>>();
        t.sort_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(&b.0);
            }
            b.1.cmp(&a.1)
        });
        t.iter().map(|v| v.0).collect::<Vec<i32>>()[0..k as usize].to_vec()
    }
}



fn main() {
    println!("Hello, world!");
}

// 奖励最顶尖的k名学生
/*
给你两个字符串数组 positive_feedback 和 negative_feedback ，分别包含表示正面的和负面的词汇。不会 有单词同时是正面的和负面的
一开始，每位学生分数为 0 。每个正面的单词会给学生的分数 加 3 分，每个负面的词会给学生的分数 减  1 分
给你 n 个学生的评语，用一个下标从 0 开始的字符串数组 report 和一个下标从 0 开始的整数数组 student_id 表示，其中 student_id[i] 表示这名学生的 ID ，这名学生的评语是 report[i] 。每名学生的 ID 互不相同
给你一个整数 k ，请你返回按照得分 从高到低 最顶尖的 k 名学生。如果有多名学生分数相同，ID 越小排名越前
*/