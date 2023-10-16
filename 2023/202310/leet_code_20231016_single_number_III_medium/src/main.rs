struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in nums {
            println!("{}", i);
            if let Some(x) = map.get_mut(&i) {
                *x = *x + 1
            } else {
                map.insert(i, 1);
            }
        }
        let mut res = Vec::new();
        for (key, val) in map.iter() {
            if *val == 1 {
                res.push(*key);
            }
        }
        res
    }
    pub fn single_number_medium(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut mp = HashMap::new();
        for num in nums {
            let count = mp.entry(num).or_insert(0);
            *count += 1;
        }
        mp.retain(|_, v|*v==1);
        mp.into_keys().collect::<Vec<i32>>()
    }
}

fn main() {
    println!("Hello, world!");
    {
        let nums = vec![1, 2, 1, 3, 2, 5];
        let s = Solution::single_number(nums);
        println!("{:?}", s);
    }
    {
        let nums = vec![1, 2, 1, 3, 2, 5];
        let s = Solution::single_number_medium(nums);
        println!("{:?}", s);
    }

}
