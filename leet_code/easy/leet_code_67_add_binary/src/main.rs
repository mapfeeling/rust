struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        format!(
            "{:b}",
            i128::from_str_radix(&a, 2).unwrap() + i128::from_str_radix(&b, 2).unwrap()
        ).to_string()
    }
}

fn main() {
    let s =Solution::add_binary(String::from("1101"),String::from("111"));
    println!("{:?}",s);
    println!("Hello, world!");
}
