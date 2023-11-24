struct Solution {}

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut i = 0;
        while i < n - 1 {
            if nums[i] == nums[i + 1] {
                ans += 1;
                i += 1;
            } else {
                i += 2;
            }
        }
        println!("{}", ans);
        ans += (n - ans) % 2;
        ans as i32
    }
}



fn main() {
    println!("Hello, world!");
    let mut nums = Vec::new();
    nums.push(1);
    nums.push(1);
    nums.push(2);
    nums.push(2);
    nums.push(3);
    nums.push(3);
    let s = Solution::min_deletion(nums);
    print!("{:?}",s);
}
