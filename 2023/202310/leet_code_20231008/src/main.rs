use std::collections::HashMap;
use std::collections::BTreeMap;

struct StockPrice {
    cur: i32,
    time_price: HashMap<i32, i32>,
    price_cnt: BTreeMap<i32, i32>,
}

impl StockPrice {
    fn new() -> Self {
        StockPrice {
            cur: 0,
            time_price: HashMap::new(),
            price_cnt: BTreeMap::new(),
        }
    }

    fn update(&mut self, time_stamp: i32, price: i32) {
        self.cur = self.cur.max(time_stamp);
        if let Some(old_price) = self.time_price.insert(time_stamp, price) {
            *self.price_cnt.entry(old_price).or_insert(0) -= 1;
            if self.price_cnt[&old_price] <= 0 {
                self.price_cnt.remove(&old_price);
            }
        }
        *self.price_cnt.entry(price).or_insert(0) += 1;
    }

    fn current(&self) -> i32 {
        self.time_price[&self.cur]
    }

    fn maximum(&self) -> i32 {
        *self.price_cnt.iter().rev().next().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.price_cnt.iter().next().unwrap().0
    }
}

fn main() {
    println!("Hello, world!");
}


/*
给你一支股票价格的数据流。数据流中每一条记录包含一个 时间戳 和该时间点股票对应的 价格
不巧的是，由于股票市场内在的波动性，股票价格记录可能不是按时间顺序到来的。某些情况下，有的记录可能是错的
如果两个有相同时间戳的记录出现在数据流中，前一条记录视为错误记录，后出现的记录 更正 前一条错误的记录
请你设计一个算法，实现：
更新 股票在某一时间戳的股票价格，如果有之前同一时间戳的价格，这一操作将 更正 之前的错误价格
找到当前记录里 最新股票价格 。最新股票价格 定义为时间戳最晚的股票价格
找到当前记录里股票的 最高价格
找到当前记录里股票的 最低价格
*/