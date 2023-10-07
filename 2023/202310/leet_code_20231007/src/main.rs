#[derive(Default)]
struct StockSpanner{
    stack:Vec<Vec<i32>>
}

impl StockSpanner {
    fn new()->Self {
        Self::default()
    }
    fn next(&mut self,price:i32) -> i32{
        let mut ret=1;
        while !self.stack.is_empty()&&self.stack[self.stack.len()-1][0]<=price {
            ret+=self.stack.pop().unwrap();
        }
        self.stack.push(vec![price, ret]);
        ret
    }
}
fn main() {
    println!("Hello, world!");
}

/*
设计一个算法收集某些股票的每日报价,并返回该股票当日价格的 跨度
当日股票价格的 跨度 被定义为股票价格小于或等于今天价格的最大连续日数（从今天开始往回数，包括今天）
例如,如果未来 7 天股票的价格是 [100,80,60,70,60,75,85]，那么股票跨度将是 [1,1,1,2,1,4,6]
实现 StockSpanner 类
StockSpanner() 初始化类对象
int next(int price) 给出今天的股价 price,返回该股票当日价格的 跨度
*/