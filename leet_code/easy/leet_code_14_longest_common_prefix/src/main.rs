struct Solution{}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.iter().max().unwrap().chars().zip(strs.iter().min().unwrap().chars()).take_while(|x|x.0==x.1).map(|x|x.0).collect()
    }
}
fn main() {
    //let  strs = Vec::new();
    let strs = vec![String::from("flower"),String::from("flow"),String::from("flight")];
    let s = Solution::longest_common_prefix(strs);
    println!("{:?}",s);
    println!("Hello, world!");
}
