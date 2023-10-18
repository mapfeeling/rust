struct Solution {}

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::from(nums);
        let mut result = 0i64;
        for _ in 0..k {
            let max = heap.pop().unwrap();
            result += max as i64;
            heap.push((max + 2) / 3);// (x+2)/3 等价于 [x/3]
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
    let mut nums = vec![1,10,3,3,3];
    let res = Solution::max_kelements(nums,3);
    println!("{:?}",res)

}


/*
给你一个下标从 0 开始的整数数组 nums 和一个整数 k 。你的 起始分数 为 0 。
在一步 操作 中：
选出一个满足 0 <= i < nums.length 的下标 i ，
将你的 分数 增加 nums[i] ，并且
将 nums[i] 替换为 ceil(nums[i] / 3) 。
返回在 恰好 执行 k 次操作后，你可能获得的最大分数。
向上取整函数 ceil(val) 的结果是大于或等于 val 的最小整数
*/