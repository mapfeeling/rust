extern crate serde_yaml;
extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    mi_dun: MiDun,
    mysql_db: MysqlDb,
    boss_price_url: String,
    boss_coupon_url: String,
    boss_redeem_url: String,
    boss_member_product_url: String,
    rocket_mq: RocketMq,

}

#[derive(Debug, Serialize, Deserialize)]
struct MiDun {
    public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MysqlDb {
    host: String,
    name: String,
    user: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RocketMq {
    name_servers: Vec<String>,
    retry: i32,
    group_name: String,
    access_key: String,
    secret_key: String,
    material_topic: String,
    position_topic: String,
    rule_topic: String,
}