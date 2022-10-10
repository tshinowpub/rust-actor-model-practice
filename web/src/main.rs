use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

mod controllers;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .route("/health", web::get().to(controllers::health_controller::index))
            .service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
