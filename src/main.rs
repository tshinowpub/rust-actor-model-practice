use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("Ok!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(health)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
