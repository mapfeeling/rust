use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut cnt = HashMap::new();
        for p1 in &points {
            cnt.clear();
            for p2 in &points {
                let d2 = (p1[0] - p2[0]) * (p1[0] - p2[0]) + (p1[1] - p2[1]) * (p1[1] - p2[1]);
                let mut v = cnt.entry(d2).or_insert(0);
                ans += *v * 2;
                *v += 1;
            }
        }
        ans
    }
}



fn main() {
    //let points:[[i32; 2]; 3] = [[0,0],[1,0],[2,0]];
    let points: Vec<Vec<i32>> = Vec::new();
    let points = vec![vec![0, 0], vec![1, 0], vec![2, 0]];
    let s = Solution::number_of_boomerangs(points);
    println!("{:?}", s);
    println!("Hello, world!");
}
