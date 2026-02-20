use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;
use actix_cors::Cors;

#[derive(Serialize, sqlx::FromRow)]
struct Message {
    id: i32,
    text: String,
}

#[get("/messages")]
async fn get_messages(db_pool: actix_web::web::Data<sqlx::PgPool>) -> impl Responder {
    let rows = sqlx::query_as::<_, Message>("SELECT id, text FROM messages")
        .fetch_all(db_pool.get_ref())
        .await;

    match rows {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().body("Error fetching messages")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    println!("Server running on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(get_messages)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}