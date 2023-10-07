fn main() {
    // rust提供了 3 种循环结构：loop 、while、for

    println!("Hello, world!");
    {
        // loop ：反复执行一块代码，直到条件满足（break）或者我们强制退出！！
        let mut count = 0;
        let res = loop {
            count += 1;
            println!("count: {}", count);
            if count == 10 {
                break count;
            }
        };
    }

    {
        // while 用法和其他语言一样
        let arr = [1, 2, 3, 4, 5];
        let mut l = arr.len();
        while l > 0 {
            l -= 1;
            println!("{}", arr[l]);
        }
    }

    {
        // for 循环：推荐使用简洁又高效；rust最为常用
        let arr = [1, 2, 3, 4, 5];
        for x in arr {
            println!("{}", x);
        }
    }

    {
        // Range：用来生成数字序列！
        for number in (10..20).rev() {
            println!("{}", number)
        }
    }
}
