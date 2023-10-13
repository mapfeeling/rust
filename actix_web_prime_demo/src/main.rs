mod init;

use crate::init::MyStruct;
use crate::init::yaml_init::AppSettings;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


// curl "127.0.0.1:8080"
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// curl "127.0.0.1:8080/echo" -H "Content-Type: application/json" -d "{"name":"dong"}"
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// curl "127.0.0.1:8080/hey"
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let yaml_str = include_str!("/Users/hcl/work/xiaomi/myGo/mapfeeling/rust/actix_web_prime_demo/config/string/app.yaml");
    let app_settings: AppSettings = serde_yaml::from_str(yaml_str).expect(".yaml read failed----->");
    let _ms = MyStruct{};
    println!("{:?}", app_settings);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

