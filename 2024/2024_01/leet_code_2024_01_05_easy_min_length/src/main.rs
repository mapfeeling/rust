struct Solution{}
impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut result = s;
        while result.contains("AB")||result.contains("CD"){
            result = result.replace("AB", "").replace("CD","");
        }
        return result.len() as i32;
    }

}
fn main() {
    println!("Hello, world!");
    let str = String::from("ABFCACDB");
    let s = Solution::min_length(str);
    println!("{:?}",s);
}
