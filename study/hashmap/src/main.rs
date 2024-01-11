use std::collections::HashMap;
use std::os::unix::raw::ino_t;

fn main() {
    // HashMap
    {
        // HashMap 的创建
        let mut languages = HashMap::new();
        languages.insert("rust", 1);
        languages.insert("go", 2);
        languages.insert("python", 3);
        println!("{:?}", languages);
    }

    {
        let name = vec!["rust", "go", "python"];
        let count = vec![1, 2, 3];
        let mut languages: HashMap<_, _> = name.iter().zip(count.iter()).collect();
        println!("{:?}", languages);
    }

    {
        // HashMap的更新
        // 直接更新覆盖
        let mut languages = HashMap::new();
        languages.insert("rust", 1);
        languages.insert("go", 2);
        languages.insert("python", 3);
        languages.insert("python", 4);
        println!("{:?}", languages);
    }

    {
        // HashMap的更新
        // 选择更新插入
        let mut languages = HashMap::new();
        languages.insert(String::from("rust"), 1);
        languages.insert(String::from("go"), 2);
        languages.insert(String::from("python"), 3);
        languages.entry(String::from("java")).or_insert(4); // key不存在才更新
    }

    {
        // 基于旧值更新
        let test = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in test.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}",map);
    }

    {
        // HashMap访问
        let mut languages = HashMap::new();
        languages.insert(String::from("rust"),1);
        languages.insert(String::from("go"),2);
        languages.insert(String::from("python"),3);
        let rust = languages.get(&String::from("rust"));

        match rust {
            None => println!{"Node"},
            Some(rust) =>  println!("{:?}",rust),
        }
    }

    {
        // HashMap遍历
        let mut languages = HashMap::new();
        languages.insert(String::from("rust"),1);
        languages.insert(String::from("go"),2);
        languages.insert(String::from("python"),3);
        for (key,value) in &languages{
            println!("the {} age is {}",key,value);
        }
    }

    {
        // HashMap所有权
        let mut languages = HashMap::new();
        let language_name = String::from("Rust");
        let language_rank= 1;
        // languages.insert(language_name,language_rank); 有错误
        languages.insert(&language_name,language_rank);
        println!("the {} rank is {}",language_name,language_rank);
    }

    println!("Hello, world!");
}
