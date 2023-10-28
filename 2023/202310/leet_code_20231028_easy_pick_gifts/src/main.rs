use std::collections::BinaryHeap;

struct Solution {}
// 2558 从礼物最多的堆取走礼物
impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heaps = BinaryHeap::from(gifts);
        for _ in 0..k {
            let top = heaps.pop().unwrap();
            heaps.push((top as f64).sqrt() as i32);
            if *heaps.peek().unwrap()==1{
                break;
            }
        }
        heaps.iter().map(|&x| x as i64).sum()
    }
}

fn main() {
    println!("Hello, world!");
    let s = Solution::pick_gifts(vec![25,64,9,4,100],4);
    println!("{:?}",s);
}


/*
给你一个整数数组 gifts ,表示各堆礼物的数量。每一秒，你需要执行以下操作：
选择礼物数量最多的那一堆
如果不止一堆都符合礼物数量最多，从中选择任一堆即可
选中的那一堆留下平方根数量的礼物（向下取整），取走其他的礼物
返回在 k 秒后剩下的礼物数量
*/
