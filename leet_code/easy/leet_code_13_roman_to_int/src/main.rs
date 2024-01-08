use std::collections::HashMap;

struct Solution{}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = HashMap::new();
        map.insert("I",1);
        map.insert("V",5);
        map.insert("X",10);
        map.insert("L",50);
        map.insert("C",100);
        map.insert("D",500);
        map.insert("M",1000);
        for i in 0..s.len(){

        }
        0
    }
}
fn main() {
    println!("Hello, world!");
}
