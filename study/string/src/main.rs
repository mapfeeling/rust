fn main() {
    // 不可变的 分配在栈上的
    let s = "Hello, world!";
    // 字符串字面量是分配在栈上的不可变，而String是分配堆上的是可变的！！！
    let _s = String::from("Hello, world2!");
    println!("{}", _s);
    {
        // 变量和数据交的方式：移动（Move)
        // 多个变量可以与同一个数据使用独特的方式来交互
        let s1 = String::from("shenyang");
        let s2 = s1;//在这里这样，rust会废弃s1的所有权，s1的值被移动到s2中，s1的值被清空
        //println!("{}",s1);//这里使用报错，因为s1已经被废弃了
        /*let s1="shenyang";
        let s2=s1;
        像这样就可以，不会报错！！！
        */
        println!("{}", s2);
        // 一个String 由3部分组成：
        // 一个指针，len（长度）,cap（容量）分配在栈上，而字符串的内容被分配在堆上
    }

    {
        let s1 = String::from("shenyang");
        let s2 = s1.clone();
        println!("{},{}", s1, s2);
    }

    {
        let mut s = String::from("Rust is a");
        s.push_str(" programming language");
        println!("{:?}", s);
        let s2 = String::from("!");
        // 字符串的拼接
        let s_other = s + &s2;
        println!("{:?\n}", s_other);
    }

    {
        let s1 = String::from("Rust");
        let s2 = &s1[1..3];
        println!("{}", s2);
        // 迭代字符串
        for i in s1.chars() {
            println!("{}", i);
        }
    }

    {
        // Rust中string的内置函数
        let s = String::from("Rust");
        //s.chars()  == s.as_bytes().iter()
        println!("{}", s.chars().count());
        println!("{}", s.len());
        println!("{:?}", s.chars().min());
        println!("{:?}", s.chars().max());
        println!("{:?}", s.chars().next());
        println!("{:?}", s.as_bytes()); // [82, 117, 115, 116]
        println!("{:?}", s.contains("R")); // true
    }

    {
        // 修改字符串的值
        let mut s = String::from("Rust");
        s.remove(0);
        s.insert(0, 'X');
        println!("{}", s);
    }

    {
        // 更高效的方式-修改字符串的值
        let mut s = "Rust".to_string();
        unsafe {
            let s_bytes: &mut [u8] = s.as_bytes_mut();
            s_bytes[0] = 'X' as u8;
            println!("s new = {}",s);
        }
    }
}

/*
Rust包含两种类型的字符串：&str和String
String
字符串被编码为UTF-8序列。在堆内存上分配一个字符串。字符串的大小可以增长。它不是以空(null)值终止的序列
&str
&str也称为字符串切片。它由&[u8]表示，指向UTP-8序列。＆str用于查看字符串中的数据。它的大小是固定的，即它不能调整大小
String 和 &str 的区别
String是一个可变引用，而&str是对该字符串的不可变引用，即可以更改String的数据，但是不能操作&str的数据
String包含其数据的所有权，而&str没有所有权，它从另一个变量借用它
*/


// 所有权是Rust最独特的特性核心特性
// 内存是通过所有权系统来管理的
// 堆和栈是代码在运行时可以傅 的内存空间
// stack 栈  这上面的数据必须拥有固定的大小
// heap 堆  编译时大小未知或者大小可能发生变化的数据必须存放在 heap中
// 访问heap中的数据要比访问stack中的数据慢，多了次指针跳转

/*
所有权规则：
1、rust中的每一个值都有一个对应的变量作为它的所有者；
2、在同一时间内,值有且仅有一个所有者；
3、当所有者离开自己的作用域时,它持有的值就会被释放掉；
*/

/*
内存与分配
rust 在变量离开作用域的时候，会调用一个叫作 drop的特殊函数
rust会在作用域结束的地方自动调用 drop 函数
*/