mod db;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::Client;
use db::{create_mongo_client, create_user, find_user_by_username, User};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_client = create_mongo_client().await.expect("Failed to connect to MongoDB");
    let mongo_data = web::Data::new(mongo_client);

    HttpServer::new(move || {
        App::new()
            .app_data(mongo_data.clone()) // Pass the client to the application
            .route("/", web::get().to(index))
            .route("/create_user", web::post().to(create_user_handler))
            .route("/get_user/{username}", web::get().to(get_user_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn create_user_handler(data: web::Data<Client>, web::Json(user_data): web::Json<User>) -> impl Responder {
    let result = create_user(&data, &user_data.username, &user_data.email , &user_data.password).await;
    match result {
        Ok(_) => HttpResponse::Created().body("User created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn get_user_handler(data: web::Data<Client>, username: web::Path<String>) -> impl Responder {
    let result = find_user_by_username(&data, &username).await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
