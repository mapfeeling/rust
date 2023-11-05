use std::collections::HashMap;
use std::os::unix::raw::ino_t;

struct Solution {}

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods = vec![0; 10];
        for e in rings.chars().collect::<Vec<_>>().chunks(2) {
            let color = match e[0] {
                'R' => 1,
                'G' => 2,
                'B' => 4,
                _ => 0,
            };

            rods[(e[1] as u8 - '0' as u8) as usize] |= color;
        }
        rods.iter().filter(|&&n| n == 7).count() as i32
    }
}

fn main() {
    println!("Hello, world!");
    let str = String::from("B0R0G0R9R0B0G0");
    let s = Solution::count_points(str);
    println!("{:?}",s);
}
/*
总计有 n 个环，环的颜色可以是红、绿、蓝中的一种。这些环分别穿在 10 根编号为 0 到 9 的杆上
给你一个长度为 2n 的字符串 rings ，表示这 n 个环在杆上的分布。rings 中每两个字符形成一个 颜色位置对 ，用于描述每个环：
第 i 对中的 第一个 字符表示第 i 个环的 颜色（'R'、'G'、'B'）
第 i 对中的 第二个 字符表示第 i 个环的 位置，也就是位于哪根杆上（'0' 到 '9'）
例如，"R3G2B1" 表示：共有 n == 3 个环，红色的环在编号为 3 的杆上，绿色的环在编号为 2 的杆上，蓝色的环在编号为 1 的杆上
找出所有集齐 全部三种颜色 环的杆，并返回这种杆的数量
*/