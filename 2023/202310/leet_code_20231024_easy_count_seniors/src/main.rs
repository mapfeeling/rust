struct Solution {}

impl Solution {
    pub fn count_seniors(details: Vec<&str>) -> i32 {
        let mut count = 0;
        for detail in details.iter() {
            if let Ok(age) = detail[11..13].parse::<i32>() {
                if age > 60 {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn count_seniors_user_map(details: Vec<&str>) -> i32 {
        details.into_iter().filter(|s| &s[11..=12] > "60").count() as i32
    }

}

fn main() {
    println!("Hello, world!");
    let s = Solution::count_seniors(vec!["7868190130M7522", "5303914400F9211", "9273338290F4010"]);
    let s_use_map = Solution::count_seniors_user_map(vec!["7868190130M7522", "5303914400F9211", "9273338290F4010"]);
    println!("{:?}", s);
    println!("{:?}", s_use_map);
}
