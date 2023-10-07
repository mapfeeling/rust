use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let num_rand=rand::thread_rng().gen_range(1..101);//左闭右合区间

    loop{//循环起始位置
        println!("Please input your guess:");
        let mut guess =String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess:u32=match guess.trim().parse(){
            Ok(num_rand) => num_rand,
            Err(_) => continue,//不需要错误信息可以下划线忽略
        };//这里把 expect方法换成了match表达式

        println!("You guessed: {}", guess);

        println!("随机数字为：{}",num_rand);

        match guess.cmp(&num_rand){
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("to big!"),
            Ordering::Equal => {
                println!("you win!");
                break;//猜对了就退出
            }
        }
    }
}
