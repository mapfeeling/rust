struct Solution {}

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let max_h = Self::get_max_size(h, horizontal_cuts);
        let max_w = Self::get_max_size(w, vertical_cuts);
        (max_h as i64 * max_w as i64 % 1_000_000_007) as i32
    }

    fn get_max_size(size: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.sort_unstable();
        let mut res = cuts[0].max(size - cuts[cuts.len() - 1]);
        for i in 1..cuts.len() {
            res = res.max(cuts[i] - cuts[i - 1]);
        }
        res
    }

}

fn main() {
    println!("Hello, world!");
    let s = Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]);
    println!("{:?}", s);
}

/*
矩形蛋糕的高度为 h 且宽度为 w，给你两个整数数组 horizontalCuts 和 verticalCuts，其中：
 horizontalCuts[i] 是从矩形蛋糕顶部到第  i 个水平切口的距离
verticalCuts[j] 是从矩形蛋糕的左侧到第 j 个竖直切口的距离
请你按数组 horizontalCuts 和 verticalCuts 中提供的水平和竖直位置切割后，请你找出 面积最大 的那份蛋糕，并返回其 面积 。由于答案可能是一个很大的数字，因此需要将结果 对 109 + 7 取余 后返回
*/