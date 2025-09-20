use actix_web::{App, HttpServer, web};
use sqlx::mysql::MySqlPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("")
        .await
        .expect("connect");
    HttpServer::new(move || App::new().app_data(web::Data::new(pool.clone())))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
