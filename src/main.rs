use actix_web::{
    App, HttpServer,
    web::{self},
};
use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
mod handler;
mod routes;
mod validation;
mod model;


//main fn actix
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    //ambil db url di env
    let db_url = env::var("DATABASE_URL").expect("ENV ERR");

    //Pool db atau koneksi db
    let pool =  MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("connect");
    //koneksi server
    println!("server berjalan pada http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
