#[warn(dead_code)]
fn main() {
    println!("hello world!");
    //默认不可变
    let mut x = 5;
    println!("x= {}", x);
    x = 10;
    println!("x= {}", x);

    //常量:它不可以使用mut关键字，常量永远都是不可变的
    //声明一个常量使用const关键字
    //常量只可以绑定到常量表达式
    //RUST中常量 一般使用大写字母
    const MAX_POINTS: u32 = 1000;


//Shadowing
    let x = 15;
    println!("{}", &x);//5
    let x = x + 1;
    println!("{}", &x);//6
    let x = x + 2;
    println!("{}", &x);//8

    let spaces_str = "      ";
    let spaces_num = spaces_str.len();
    println!("{}", spaces_num);//6

    {
        /*标量类型：整数，浮点数（f32,f64默认的），布尔值，字符（char）,字符串，元组，枚举
        复合类型：数组，结构体，指针，元组，枚举
        u32:无符号整数类型，占32位空间
        u8,u16,u32,u64,u128 无符号
        i8,i16,i32,i64,i128 有符号
        无符号以U开头，有符号以I开头
        整数默认类型是i32
        */
        //isize 和 usize 这两种是由运算程序的计算机硬件决定的
        let guess:u32 = "42".parse().expect("Not a number!");
        println!("{}",guess);
        //rust声明的变量没有使用会有警告
        let x:f32 = 3.0;
        let x:f64 = 4.0;
        let b:bool = true;
        let b:bool =false;
        let x='z';
        let y:char ='y';
        let z='😀';//也可以存放这种

    }
    {
        //复合类型
        //Tuple类型可以将多个类型的值放在一个类型里面，和C++中的元组类似
        //tuple的长度的固定，一旦创建就不能改变
        //如果不明确是什么类型，可以使用_来代替
        let tup: (i32,char,bool)=(15,'a',true);
        let tup:(_,_,bool)=(3.14,"boy",false);
        let tup=(50,1.25,1);//也可以使用模式匹配
        let ont=tup.0;
        let two=tup.1;//也可以通过点来进行访问
        let (x,y,z)=tup;
        println!("{},{},{}",x,y,z);//获取tup的值 解构：将元组拆解为n个不同的部分
        //数组
        //数组和C++中的差不多
        //长度也是固定的
        // Vertor更加灵活，长度可以改变
        //和数组类似，不确定使用哪个，就使用 Vector
        let arr=[1,2,3,4,5,6,7,8,9];
        let avec=vec![1,2,3,4,5,6,7,8,9];
        println!("{}",avec[0]);
        println!("{}",arr[5]);
        //另外一声明数组的方法
        let a=[3;5]; //创建数组并且初始化为5个3
    }
    {
        // 函数调用
        add_function();
        add(4,56);

        let y=5+6;

        let y={//表达式
            let x=3;
            x+1 //这个加上了分号就变成了语句；这个相当于返回值
        };

        let n1={
            let u=6+5;
            u
        };


        println!("{}",y);//4

        //函数和注释
        // 声明函数使用  fn 关键字 : go语言使用 func 关键字
        // 规范是函数名称使用小写，单词之间使用_分割
        fn add_function(){
            println!("hello function");
        }

        fn add(x:u32 , y: u32){//rust必须指定函数参数类型
            println!("x={}",x);
            println!("y={}",y);
        }
        // 函数的返回值
        fn add1(x:u32 , y:u32) ->u32 {//在参数括号后面加上->类型 就是返回
            let x=x+y;
            x   //返回语句不能有分号，有了分号就变成了语句
        }
    }

}