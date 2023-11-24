struct Solution {}

impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut ans: i32 = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] >= target {
                    break;
                }
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
    let s = Solution::count_pairs(vec![-6,2,5,-2,-7,-1,3],2);
    println!("{:?}",s);
}
