struct Solution {}
// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引
// 如果目标值不存在于数组中，返回它将会被按顺序插入的位置
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}

fn main() {
    let s = Solution::search_insert(vec![1,3,5,6],5);
    println!("{:?}",s);
    println!("Hello, world!");
}
