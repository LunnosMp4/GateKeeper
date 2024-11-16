use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

