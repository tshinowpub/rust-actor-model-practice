use actix_web::{get, App, HttpServer, HttpResponse, web};

mod controllers;

#[get("/getjson")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"str":"テスト１","num":100,"arr":[1,2,3]}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(web::resource("/channels/{id}").route(web::get().to(controllers::channel_controller::detail)))
            .route("/channels", web::get().to(controllers::channel_controller::index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
