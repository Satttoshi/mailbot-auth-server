mod db;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use db::create_mongo_client;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_client = create_mongo_client().await.expect("Failed to connect to MongoDB");

    HttpServer::new(move || {
        App::new()
            .app_data(mongo_client.clone()) // Pass the client to the application
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
